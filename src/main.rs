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

#![windows_subsystem = "windows"]

use desktop::app::App;
use iced::{window, Application, Settings};

pub fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            size: (1028, 640),
            min_size: Some((1028, 640)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
