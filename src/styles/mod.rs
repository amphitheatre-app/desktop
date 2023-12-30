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

pub mod card;
pub mod constants;

mod container;
pub use container::Container;

pub mod modal;
pub mod rule;
pub mod scrollable;
pub mod split;
pub mod tab_bar;
pub mod text_input;

mod text;
pub use text::Text;

/// The custom theme for the application.
/// [iced example](https://github.com/iced-rs/iced/blob/master/examples/styling/src/main.rs)
/// [apple color guidelines](https://developer.apple.com/design/human-interface-guidelines/color)
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    background: iced::Color,
    text: iced::Color,
    primary: iced::Color,
    success: iced::Color,
    danger: iced::Color,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            background: iced::Color::from_rgba8(30, 30, 30, 1.0),
            text: iced::Color::from_rgba8(221, 221, 221, 1.0),
            primary: iced::Color::from_rgba8(10, 132, 255, 1.0),
            success: iced::Color::from_rgba8(48, 209, 81, 1.0),
            danger: iced::Color::from_rgba8(255, 69, 58, 1.0),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}
