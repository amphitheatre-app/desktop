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

use crate::context::Context;
use crate::errors::{Errors, Result};
use std::sync::Arc;

pub async fn switch_context(ctx: Arc<Context>, name: String) -> Result<()> {
    let mut configuration = ctx.configuration.write().unwrap();
    let context = configuration.context.as_mut().unwrap();

    context
        .select(&name)
        .map_err(|e| Errors::FailedSelectContext(e.to_string()))?;

    let path = Configuration::path().map_err(|e| Errors::InvalidConfigPath(e.to_string()))?;
    configuration
        .save(path)
        .map_err(|e| Errors::FailedSaveConfiguration(e.to_string()))?;

    Ok(())
}
