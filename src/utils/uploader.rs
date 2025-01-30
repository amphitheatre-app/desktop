// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use tar::Builder;
use tracing::debug;

use amp_common::sync::{EventKinds, Synchronization};

use crate::{
    context::Context,
    errors::{Errors, Result},
};

/// Upload the given directory to the server.
pub async fn upload(ctx: &Context, pid: &str, actor: &str, workspace: &Path) -> Result<()> {
    let mut paths: Vec<(PathBuf, PathBuf)> = vec![];

    let base = workspace;
    for entry in WalkBuilder::new(workspace).build() {
        let entry = entry.map_err(Errors::WalkError)?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        paths.push(strip(base, path)?);
    }

    let payload = archive(&paths)?;
    let req = Synchronization {
        kind: EventKinds::Overwrite,
        paths: vec![],
        attributes: None,
        payload: Some(payload),
    };
    ctx.client()
        .actors()
        .sync(pid, actor, req)
        .await
        .map_err(|e| Errors::ClientError(e.to_string()))?;

    Ok(())
}

/// Archive the given directory into a tarball and return the bytes.
pub fn archive(paths: &Vec<(PathBuf, PathBuf)>) -> Result<Vec<u8>> {
    debug!("The given path for archive is {:?}", paths);
    let mut tar = Builder::new(Vec::new());
    for (path, name) in paths {
        tar.append_path_with_name(path, name)
            .map_err(|e| Errors::FailedAppendPath(e.to_string()))?;
    }
    tar.into_inner().map_err(|e| Errors::FailedFinishTar(e.to_string()))
}

/// Strip the given base path from the given path.
#[inline]
pub fn strip(base: &Path, path: &Path) -> Result<(PathBuf, PathBuf)> {
    let striped_path = path
        .strip_prefix(base)
        .map_err(|e| Errors::FailedStripPrefix(e.to_string()))?;
    debug!("the full path and striped path is: {:?}, {:?}", path, striped_path);
    Ok((path.to_path_buf(), striped_path.to_path_buf()))
}
