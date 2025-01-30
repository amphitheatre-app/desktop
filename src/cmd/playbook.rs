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
use std::path::PathBuf;

use crate::context::Context;
use crate::errors::{Errors, Result};
use crate::utils::{uploader, watcher};

use amp_client::playbooks::PlaybookPayload;
use amp_common::resource::PlaybookSpec;
use amp_common::{
    resource::{CharacterSpec, Preface},
    schema::Character,
};
use tracing::{debug, error, info};

pub async fn refresh_playbooks(ctx: Context) -> Result<Vec<PlaybookSpec>> {
    ctx.client()
        .playbooks()
        .list(None)
        .await
        .map_err(|e| Errors::ClientError(e.to_string()))
}

pub async fn compose(
    ctx: Context,
    title: impl ToString,
    description: impl ToString,
    preface: impl ToString,
    live: bool,
) -> Result<PlaybookSpec> {
    let playbook: PlaybookSpec;

    if preface.to_string().starts_with("http") {
        playbook = pull(&ctx, title, description, preface).await?;
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

        playbook = load(&ctx, title, description, &character).await?;
        sync(&ctx, playbook.clone(), actor, &workspace, live).await?;
    };

    Ok(playbook)
}

/// Create a playbook from the remote git repository.
async fn pull(
    ctx: &Context,
    title: impl ToString,
    description: impl ToString,
    repository: impl ToString,
) -> Result<PlaybookSpec> {
    create(
        ctx,
        PlaybookPayload {
            title: title.to_string(),
            description: description.to_string(),
            preface: Preface::repository(&repository.to_string()),
        },
    )
    .await
}

/// Create a playbook from the local manifest file.
#[allow(dead_code)]
async fn load(
    ctx: &Context,
    title: impl ToString,
    description: impl ToString,
    character: &CharacterSpec,
) -> Result<PlaybookSpec> {
    create(
        ctx,
        PlaybookPayload {
            title: title.to_string(),
            description: description.to_string(),
            preface: Preface::manifest(character),
        },
    )
    .await
}

async fn sync(ctx: &Context, playbook: PlaybookSpec, actor: &str, workspace: &Path, live: bool) -> Result<()> {
    info!("Syncing the full sources into the server...");
    uploader::upload(ctx, &playbook.id, actor, workspace).await?;

    if !live {
        return Ok(());
    }

    let workspace = workspace.to_path_buf();
    let actor = actor.to_string().clone();
    let client = ctx.client();
    let pid1 = playbook.id.clone();

    info!("Watching file changes and sync the changed files.");
    tokio::spawn(async move {
        if let Err(err) = watcher::watch(&workspace, &client, &pid1, &actor).await {
            error!("The watcher is stopped: {:?}", err);
        }
    });

    Ok(())
}

/// Create a playbook from the given payload.
async fn create(ctx: &Context, payload: PlaybookPayload) -> Result<PlaybookSpec> {
    let playbook = ctx
        .client()
        .playbooks()
        .create(payload)
        .await
        .map_err(|e| Errors::FailedCreatePlaybook(e.to_string()))?;

    info!("The playbook begins to create...");
    debug!("The created playbook is:\n {:#?}", playbook);

    Ok(playbook)
}

pub async fn close_playbook(ctx: Context, pid: impl ToString) -> Result<u16> {
    ctx.client()
        .playbooks()
        .delete(&pid.to_string())
        .await
        .map_err(|e| Errors::FailedDeletePlaybook(e.to_string()))
}
