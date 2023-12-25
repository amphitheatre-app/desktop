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

use iced::widget::button::{self, Appearance, StyleSheet};
use iced::{color, Color};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Default,
    Element,
}

impl StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            Button::Default => Appearance {
                background: Some(Color::TRANSPARENT.into()),
                border_radius: 6.0.into(),
                border_width: 1.0,
                border_color: color!(0x474B56),
                text_color: color!(0xffffff),
                ..Appearance::default()
            },
            Button::Element => Appearance::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => Appearance {
                background: Some(color!(0x474B56).into()),
                border_radius: 6.0.into(),
                border_width: 1.0,
                border_color: color!(0x474B56),
                text_color: color!(0xffffff),
                ..Appearance::default()
            },
            Button::Element => Appearance::default(),
        }
    }
}
