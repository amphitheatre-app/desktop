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

use crate::context::Context;
use crate::errors::{Errors, Result};
use std::collections::HashMap;

pub async fn refresh_actor_info(
    ctx: Context,
    pid: impl ToString,
    name: impl ToString,
) -> Result<HashMap<String, HashMap<String, String>>> {
    ctx.client()
        .actors()
        .info(&pid.to_string(), &name.to_string())
        .await
        .map_err(|e| Errors::ClientError(e.to_string()))
        .map(|data| serde_json::from_value(data).map_err(|e| Errors::SerdeJsonError(e.to_string())))?
}

pub async fn refresh_actor_stats(
    ctx: Context,
    pid: impl ToString,
    name: impl ToString,
) -> Result<HashMap<String, String>> {
    ctx.client()
        .actors()
        .stats(&pid.to_string(), &name.to_string())
        .await
        .map_err(|e| Errors::ClientError(e.to_string()))
        .map(|data| serde_json::from_value(data).map_err(|e| Errors::SerdeJsonError(e.to_string())))?
}
