// Copyright 2024 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::{Path, PathBuf};

use amp_client::client::Client;
use amp_common::sync::{self, EventKinds, Synchronization};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use notify::event::RemoveKind;
use notify::EventKind::Remove;
use notify::RecursiveMode::Recursive;
use notify::{Event, RecommendedWatcher, Watcher};
use tracing::{debug, error, trace, warn};

use crate::errors::{Errors, Result};
use crate::utils::uploader;

///  Watch file changes and sync the changed files.
pub async fn watch(workspace: &Path, client: &Client, pid: &str, name: &str) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // We listen to the file changes giving Notify
    // a function that will get called when events happen.
    let config = notify::Config::default();
    let mut watcher = RecommendedWatcher::new(tx, config).map_err(|e| Errors::FailedCreateWatcher(e.to_string()))?;
    watcher
        .watch(workspace, Recursive)
        .map_err(|e| Errors::FailedWatchDirectory(e.to_string()))?;

    let mut builder = GitignoreBuilder::new(workspace);
    builder.add(".gitignore");
    let matcher = builder.build().unwrap();

    for event in rx {
        if let Err(err) = event {
            error!("Got a notify error: {err:?}");
            continue;
        }
        let event = event.unwrap();
        if is_ignored(&matcher, workspace, &event.paths)? {
            continue;
        }

        handle(client, pid, name, workspace, event).await?;
    }

    Ok(())
}

async fn handle(client: &Client, pid: &str, name: &str, base: &Path, event: Event) -> Result<()> {
    trace!("Changed: {:?}", event);

    let kind = EventKinds::from(event.kind);
    if kind == EventKinds::Rename || kind == EventKinds::Other {
        warn!("Not supported event: {:?}", event);
        return Ok(());
    }

    let mut paths: Vec<(PathBuf, PathBuf)> = vec![];
    for path in event.paths {
        paths.push(uploader::strip(base, &path)?);
    }

    let mut req = Synchronization {
        kind: kind.clone(),
        paths: vec![],
        attributes: None,
        payload: None,
    };

    // Because the file or directory was removed yet, we can't get the file type.
    // so we determine the file type by original event kind.
    if kind == EventKinds::Remove {
        let is_dir = event.kind == Remove(RemoveKind::Folder);
        req.paths = paths.iter().map(|(_, b)| format_path(b, is_dir)).collect();
    } else {
        req.paths = paths.iter().map(|(a, b)| format_path(b, a.is_dir())).collect();
    }

    if kind == EventKinds::Modify {
        req.payload = Some(uploader::archive(&paths)?);
    }

    debug!("The sync request is: {:?}", req);
    client
        .actors()
        .sync(pid, name, req)
        .await
        .map_err(|e| Errors::ClientError(e.to_string()))?;

    Ok(())
}

fn format_path(path: &Path, is_dir: bool) -> sync::Path {
    let path_string = path.to_str().unwrap().to_string();
    match is_dir {
        true => sync::Path::Directory(path_string),
        false => sync::Path::File(path_string),
    }
}

fn is_ignored(matcher: &Gitignore, root: &Path, paths: &Vec<PathBuf>) -> Result<bool> {
    for path in paths {
        let name = path
            .strip_prefix(root)
            .map_err(|e| Errors::FailedStripPrefix(e.to_string()))?;
        if matcher.matched(name, false).is_ignore() {
            debug!("The file is ignored: {:?}", name);
            return Ok(true);
        }
    }

    Ok(false)
}
