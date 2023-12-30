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

use iced::widget::container::{Appearance, StyleSheet};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    Toolbar,
    Sidebar,
}

impl StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let background_color = match style {
            Container::Default => iced::Color::TRANSPARENT,
            Container::Toolbar => {
                let mut c = self.background;
                c.r += 0.03;
                c.g += 0.03;
                c.b += 0.03;
                c
            }
            Container::Sidebar => {
                let mut c = self.background;
                c.r += 0.05;
                c.g += 0.05;
                c.b += 0.05;
                c
            }
        };

        Appearance {
            background: Some(background_color.into()),
            ..Appearance::default()
        }
    }
}
