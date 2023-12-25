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

use amp_common::config::Configuration;
use thiserror::Error;

/// Load the configuration from the file system.
pub async fn load() -> Result<Configuration, ConfigurationError> {
    let path = Configuration::path().map_err(|_e| ConfigurationError::BadConfigDirectory)?;
    let configuration = Configuration::load(path).map_err(|_e| ConfigurationError::LoadError)?;

    Ok(configuration)
}

#[derive(Clone, Debug, Error)]
pub enum ConfigurationError {
    #[error("could not determine home directory path")]
    BadConfigDirectory,

    #[error("unable to load configuration")]
    LoadError,
}
