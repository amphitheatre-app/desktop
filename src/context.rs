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

use amp_client::client::Client;
use amp_common::config::{Cluster, Configuration};

use crate::errors::{Errors, Result};

use std::sync::RwLock;
use std::{fmt::Debug, sync::Arc};

/// Context holds the current context state
pub struct Context {
    pub configuration: RwLock<Configuration>,
    pub cluster: RwLock<Cluster>,
    pub client: Arc<Client>,
}

impl Default for Context {
    fn default() -> Self {
        let configuration = Configuration::default();
        let cluster = get_context(&configuration).unwrap_or_default();
        let client = Client::new(&format!("{}/v1", &cluster.server), cluster.token.clone());

        Self {
            configuration: RwLock::new(configuration),
            cluster: RwLock::new(cluster),
            client: Arc::new(client),
        }
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("configuration", &self.configuration)
            .field("cluster", &self.cluster)
            .finish()
    }
}

impl Context {
    /// Initialize a new context
    pub fn init() -> Result<Context> {
        let path = Configuration::path().map_err(Errors::InvalidConfigPath)?;
        let configuration = Configuration::load(path).map_err(Errors::FailedLoadConfiguration)?;
        let cluster = get_context(&configuration)?;
        let client = Client::new(&format!("{}/v1", &cluster.server), cluster.token.clone());

        Ok(Context {
            configuration: RwLock::new(configuration),
            cluster: RwLock::new(cluster),
            client: Arc::new(client),
        })
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
