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

// Prevent console window from showing up on Windows
#![windows_subsystem = "windows"]

use std::sync::Arc;

use desktop::app::App;
use desktop::context::Context;
use desktop::errors::{Errors::IcedError, Result};
use desktop::styles::constants::*;
use iced::{window, Application, Settings};
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    // Initialize tracing
    std::env::set_var("RUST_LOG", "desktop=trace");
    let filter = EnvFilter::builder().from_env_lossy();
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(filter)
        .init();

    App::run(Settings {
        flags: Arc::new(Context::init()?),
        window: window::Settings {
            size: (WINDOW_INITIAL_WIDTH, WINDOW_INITIAL_HEIGHT),
            min_size: Some((WINDOW_INITIAL_WIDTH, WINDOW_INITIAL_HEIGHT)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
    .map_err(|e| IcedError(e.to_string()))
}
