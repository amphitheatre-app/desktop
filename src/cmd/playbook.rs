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

use std::{path::PathBuf, sync::Arc};

use crate::errors::{Errors, Result};
use amp_client::playbooks::{Playbook, PlaybookPayload, Playbooks};
use amp_common::{
    resource::{CharacterSpec, Preface},
    schema::Character,
};
use tracing::{debug, info};

use crate::context::Context;

pub async fn refresh_playbooks(ctx: Arc<Context>) -> Result<Vec<Playbook>> {
    ctx.client()?
        .playbooks()
        .list(None)
        .map_err(|e| Errors::ClientError(e.to_string()))
}

pub async fn compose(
    ctx: Arc<Context>,
    title: impl ToString,
    description: impl ToString,
    preface: impl ToString,
    _live: bool,
) -> Result<Playbook> {
    if preface.to_string().starts_with("http") {
        pull(&ctx, title, description, preface)
    } else {
        Err(Errors::FailedCreatePlaybook(
            "The local manifest file is not supported yet.".to_string(),
        ))
        // load(&ctx, title, description, preface, true, !live).await
    }
}

/// Create a playbook from the remote git repository.
fn pull(
    ctx: &Context,
    title: impl ToString,
    description: impl ToString,
    repository: impl ToString,
) -> Result<Playbook> {
    create(
        ctx.client()?.playbooks(),
        PlaybookPayload {
            title: title.to_string(),
            description: description.to_string(),
            preface: Preface::repository(&repository.to_string()),
        },
    )
}

/// Create a playbook from the local manifest file.
#[allow(dead_code)]
async fn load(
    ctx: &Context,
    title: impl ToString,
    description: impl ToString,
    path: impl ToString,
    live: bool,
    once: bool,
) -> Result<Playbook> {
    let path = PathBuf::from(path.to_string()).join(".amp.toml");
    let manifest = Character::load(path).map_err(|e| Errors::FailedLoadManifest(e.to_string()))?;
    let character = CharacterSpec {
        live,
        once,
        ..CharacterSpec::from(&manifest)
    };

    return create(
        ctx.client()?.playbooks(),
        PlaybookPayload {
            title: title.to_string(),
            description: description.to_string(),
            preface: Preface::manifest(&character),
        },
    );
}

/// Create a playbook from the given payload.
fn create(client: Playbooks, payload: PlaybookPayload) -> Result<Playbook> {
    let playbook = client
        .create(payload)
        .map_err(|e| Errors::FailedCreatePlaybook(e.to_string()))?;

    info!("The playbook begins to create...");
    debug!("The created playbook is:\n {:#?}", playbook);

    Ok(playbook)
}

pub async fn close_playbook(ctx: Arc<Context>, pid: String) -> Result<u16> {
    ctx.client()?
        .playbooks()
        .delete(&pid)
        .map_err(|e| Errors::FailedDeletePlaybook(e.to_string()))
}
