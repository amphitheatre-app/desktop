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

use crate::errors::{Errors, Result};
use amp_client::client::Client;
use amp_common::config::{Cluster, Configuration};
use std::sync::{Arc, RwLock};

/// Context holds the current context state
pub struct Context {
    pub configuration: RwLock<Configuration>,
    pub client: Arc<RwLock<Client>>,
}

impl Default for Context {
    fn default() -> Self {
        let configuration = Configuration::default();
        let (_, cluster) = context(&configuration).unwrap_or_default();
        let client = Client::new(&format!("{}/v1", &cluster.server), cluster.token.clone());

        Context {
            configuration: RwLock::new(configuration),
            client: Arc::new(RwLock::new(client)),
        }
    }
}

impl Context {
    /// Initialize a new context
    pub fn init() -> Result<Context> {
        let path = Configuration::path().map_err(|e| Errors::InvalidConfigPath(e.to_string()))?;
        let configuration = Configuration::load(path).map_err(|e| Errors::FailedLoadConfiguration(e.to_string()))?;

        let (_, cluster) = context(&configuration)?;
        let client = Client::new(&format!("{}/v1", &cluster.server), cluster.token.clone());

        Ok(Context {
            configuration: RwLock::new(configuration),
            client: Arc::new(RwLock::new(client)),
        })
    }
}

/// Get the current context from the configuration
fn context(configuration: &Configuration) -> Result<(String, Cluster)> {
    if let Some(context) = &configuration.context {
        return context.current().ok_or(Errors::NotFoundCurrentContext);
    }
    Err(Errors::NotFoundCurrentContext)
}
