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

use super::Theme;
use iced::widget::button::{Appearance, StyleSheet};

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Default,
    Element,
    Icon,
    Primary,
    Menu,
}

impl StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            Button::Default => Appearance {
                text_color: self.text,
                border_width: 1.0,
                border_radius: 6.0.into(),
                border_color: iced::Color { a: 0.1, ..self.text },
                ..Appearance::default()
            },
            Button::Element => Appearance::default(),
            Button::Icon => Appearance::default(),
            Button::Primary => Appearance {
                text_color: self.text,
                border_width: 1.0,
                border_radius: 6.0.into(),
                border_color: iced::Color { a: 0.4, ..self.primary },
                background: Some(self.primary.into()),
                ..Appearance::default()
            },
            Button::Menu => Appearance::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            Button::Menu => Appearance {
                text_color: self.text,
                border_width: 0.0,
                border_radius: 6.0.into(),
                background: Some(self.primary.into()),
                ..Appearance::default()
            },
            _ => self.active(style),
        }
    }
}
