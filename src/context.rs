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

pub struct Context(Arc<RwLock<ContextInner>>);

impl Context {
    /// Initialize a new context
    pub fn init() -> Result<Context> {
        let path = Configuration::path().map_err(|e| Errors::InvalidConfigPath(e.to_string()))?;
        let configuration = Configuration::load(path).map_err(|e| Errors::FailedLoadConfiguration(e.to_string()))?;

        let (_, cluster) = current(&configuration)?;
        let client = Client::new(&format!("{}/v1", &cluster.server), cluster.token.clone());

        Ok(Context(Arc::new(RwLock::new(ContextInner {
            configuration: Arc::new(configuration),
            client: Arc::new(client),
        }))))
    }

    /// Get the readonly client
    pub fn client(&self) -> Arc<Client> {
        self.0.read().unwrap().client.clone()
    }

    /// Get the readonly configuration
    pub fn configuration(&self) -> Arc<Configuration> {
        self.0.read().unwrap().configuration.clone()
    }

    pub async fn switch(&mut self, name: String) -> Result<()> {
        // read the configuration
        let mut configuration = self.configuration();
        let configuration = Arc::make_mut(&mut configuration);
        let context = configuration.context.as_mut().unwrap();

        // switch the context
        context
            .select(&name)
            .map_err(|e| Errors::FailedSelectContext(e.to_string()))?;

        // save the configuration
        let path = Configuration::path().map_err(|e| Errors::InvalidConfigPath(e.to_string()))?;
        configuration
            .save(path)
            .map_err(|e| Errors::FailedSaveConfiguration(e.to_string()))?;

        // reload the context
        *self = Context::init()?;

        Ok(())
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context(self.0.clone())
    }
}

/// Context holds the current context state
pub struct ContextInner {
    configuration: Arc<Configuration>,
    client: Arc<Client>,
}

/// Get the current context from the configuration
fn current(configuration: &Configuration) -> Result<(String, Cluster)> {
    if let Some(context) = &configuration.context {
        return context.current().ok_or(Errors::NotFoundCurrentContext);
    }
    Err(Errors::NotFoundCurrentContext)
}
