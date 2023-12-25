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

pub mod application;

mod button;
pub use button::Button;

pub mod constants;
pub mod container;
pub mod rule;
pub mod scrollable;
pub mod split;
pub mod tab_bar;

pub mod text_input;

mod text;
pub use text::Text;

/// The custom theme for the application.
/// All the widgets will implement for this theme.
#[derive(Debug, Clone, Copy, Default)]
pub struct Theme;
