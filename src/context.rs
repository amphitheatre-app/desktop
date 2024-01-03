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

use crate::errors::{Errors, Result};
use amp_client::client::Client;
use amp_common::config::{Cluster, Configuration, ContextConfiguration};
use std::fmt::Debug;
use std::sync::RwLock;

/// Context holds the current context state
pub struct Context {
    pub configuration: RwLock<Configuration>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            configuration: RwLock::new(Configuration::default()),
        }
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("configuration", &self.configuration)
            .finish()
    }
}

impl Context {
    /// Initialize a new context
    pub fn init() -> Result<Context> {
        let path = Configuration::path().map_err(|e| Errors::InvalidConfigPath(e.to_string()))?;
        let configuration = Configuration::load(path).map_err(|e| Errors::FailedLoadConfiguration(e.to_string()))?;

        Ok(Context {
            configuration: RwLock::new(configuration),
        })
    }

    /// Get client with current context
    pub fn client(&self) -> Result<Client> {
        let cluster = get_context(&self.configuration.read().unwrap())?;
        let client = Client::new(&format!("{}/v1", &cluster.server), cluster.token.clone());
        Ok(client)
    }

    /// Get the contexts from the configuration
    pub fn contexts(&self) -> Result<ContextConfiguration> {
        let configuration = self
            .configuration
            .read()
            .map_err(|e| Errors::FailedLoadConfiguration(e.to_string()))?;
        Ok(configuration.context.clone().unwrap_or_default())
    }
}

/// Get the current context from the configuration
fn get_context(configuration: &Configuration) -> Result<Cluster> {
    if let Some(context) = &configuration.context {
        if let Some(current) = context.current() {
            return Ok(current.to_owned());
        }
    }

    Err(Errors::NotFoundCurrentContext)
}
