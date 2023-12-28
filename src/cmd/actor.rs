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

use crate::context::Context;
use std::{collections::HashMap, sync::Arc};

pub async fn refresh_actor_info(
    ctx: Arc<Context>,
    pid: impl ToString,
    name: impl ToString,
) -> HashMap<String, HashMap<String, String>> {
    ctx.client
        .actors()
        .info(&pid.to_string(), &name.to_string())
        .map(|data| serde_json::from_value(data).unwrap())
        .unwrap_or_default()
}

pub async fn refresh_actor_stats(
    ctx: Arc<Context>,
    pid: impl ToString,
    name: impl ToString,
) -> HashMap<String, String> {
    ctx.client
        .actors()
        .stats(&pid.to_string(), &name.to_string())
        .map(|data| serde_json::from_value(data).unwrap())
        .unwrap_or_default()
}
