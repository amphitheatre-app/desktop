// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

use std::path::Path;
use std::{path::PathBuf, sync::Arc};

use crate::context::Context;
use crate::errors::{Errors, Result};
use crate::utils::{uploader, watcher};

use amp_client::playbooks::{PlaybookPayload, Playbooks};
use amp_common::resource::PlaybookSpec;
use amp_common::{
    resource::{CharacterSpec, Preface},
    schema::Character,
};
use tracing::{debug, error, info};

pub async fn refresh_playbooks(ctx: Arc<Context>) -> Result<Vec<PlaybookSpec>> {
    ctx.client
        .read()
        .unwrap()
        .playbooks()
        .list(None)
        .map_err(|e| Errors::ClientError(e.to_string()))
}

pub async fn compose(
    ctx: Arc<Context>,
    title: impl ToString,
    description: impl ToString,
    preface: impl ToString,
    live: bool,
) -> Result<PlaybookSpec> {
    let playbook: PlaybookSpec;

    if preface.to_string().starts_with("http") {
        playbook = pull(&ctx, title, description, preface)?;
    } else {
        let path = PathBuf::from(preface.to_string()).join(".amp.toml");
        let workspace = path.parent().unwrap().to_path_buf();

        let manifest = Character::load(path).map_err(|e| Errors::FailedLoadManifest(e.to_string()))?;
        let actor = &manifest.meta.name;

        let character = CharacterSpec {
            live: true,
            once: !live,
            ..CharacterSpec::from(&manifest)
        };

        playbook = load(&ctx, title, description, &character)?;
        sync(ctx, playbook.clone(), actor, &workspace, live)?;
    };

    Ok(playbook)
}

/// Create a playbook from the remote git repository.
fn pull(
    ctx: &Context,
    title: impl ToString,
    description: impl ToString,
    repository: impl ToString,
) -> Result<PlaybookSpec> {
    create(
        ctx.client.read().unwrap().playbooks(),
        PlaybookPayload {
            title: title.to_string(),
            description: description.to_string(),
            preface: Preface::repository(&repository.to_string()),
        },
    )
}

/// Create a playbook from the local manifest file.
#[allow(dead_code)]
fn load(
    ctx: &Context,
    title: impl ToString,
    description: impl ToString,
    character: &CharacterSpec,
) -> Result<PlaybookSpec> {
    create(
        ctx.client.read().unwrap().playbooks(),
        PlaybookPayload {
            title: title.to_string(),
            description: description.to_string(),
            preface: Preface::manifest(character),
        },
    )
}

fn sync(ctx: Arc<Context>, playbook: PlaybookSpec, actor: &str, workspace: &Path, live: bool) -> Result<()> {
    info!("Syncing the full sources into the server...");
    uploader::upload(&ctx.client.read().unwrap().actors(), &playbook.id, actor, workspace)?;

    if !live {
        return Ok(());
    }

    let workspace = workspace.to_path_buf();
    let actor = actor.to_string().clone();
    let client = ctx.client.clone();
    let pid1 = playbook.id.clone();

    info!("Watching file changes and sync the changed files.");
    tokio::spawn(async move {
        let client = client.read().unwrap();
        if let Err(err) = watcher::watch(&workspace, &client, &pid1, &actor) {
            error!("The watcher is stopped: {:?}", err);
        }
    });

    Ok(())
}

/// Create a playbook from the given payload.
fn create(client: Playbooks, payload: PlaybookPayload) -> Result<PlaybookSpec> {
    let playbook = client
        .create(payload)
        .map_err(|e| Errors::FailedCreatePlaybook(e.to_string()))?;

    info!("The playbook begins to create...");
    debug!("The created playbook is:\n {:#?}", playbook);

    Ok(playbook)
}

pub async fn close_playbook(ctx: Arc<Context>, pid: impl ToString) -> Result<u16> {
    ctx.client
        .read()
        .unwrap()
        .playbooks()
        .delete(&pid.to_string())
        .map_err(|e| Errors::FailedDeletePlaybook(e.to_string()))
}
