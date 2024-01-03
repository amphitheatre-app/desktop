// Copyright 2023 The Amphitheatre Authors.
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

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Errors>;

#[derive(Debug, Error, Clone)]
pub enum Errors {
    #[error("Invalid configuration path")]
    InvalidConfigPath(String),

    #[error("Failed to load configuration")]
    FailedLoadConfiguration(String),

    #[error("Current context not found, please use `amp context` for help")]
    NotFoundCurrentContext,

    #[error("Client error: {0}")]
    ClientError(String),

    #[error("Failed to load manifest: {0}")]
    FailedLoadManifest(String),

    #[error("Failed to delete playbook: {0}")]
    FailedDeletePlaybook(String),

    #[error("Failed to delete context: {0}")]
    FailedDeleteContext(String),

    #[allow(dead_code)]
    #[error("Not found context: {0}")]
    NotFoundContext(String),

    #[error("Failed to save configuration")]
    FailedSaveConfiguration(String),

    #[error("Failed to serialize toml")]
    TomlSerializeError(toml::ser::Error),

    #[error("Failed to deserialize from toml")]
    SerdeJsonError(String),

    #[error("Failed to save manifest: {0}")]
    FailedSaveManifest(String),

    #[error("Failed to create playbook: {0}")]
    FailedCreatePlaybook(String),

    #[error("Failed to finish tar: {0}")]
    FailedFinishTar(String),

    #[error("Walk directory error: {0}")]
    WalkError(ignore::Error),

    #[error("Failed to strip prefix: {0}")]
    FailedStripPrefix(String),

    #[error("Failed to append path: {0}")]
    FailedAppendPath(String),

    #[error("Failed to create watcher: {0}")]
    FailedCreateWatcher(String),

    #[error("Failed to watch directory: {0}")]
    FailedWatchDirectory(String),

    #[error("Not found available contexts")]
    NotFoundContexts,

    #[error("Failed to select context: {0}")]
    FailedSelectContext(String),

    #[error("Failed to add context: {0}")]
    FailedAddContext(String),

    #[error("Some error occurred: {0}")]
    IcedError(String),
}
