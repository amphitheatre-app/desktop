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

use iced::widget::text_input::{Appearance, StyleSheet};
use iced::{color, Color};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: iced::Color::TRANSPARENT.into(),
            border_radius: 6.0.into(),
            border_width: 1.0,
            border_color: iced::Color { a: 0.1, ..self.text },
            icon_color: iced::Color { a: 0.1, ..self.text },
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        Appearance {
            border_color: self.primary,
            ..self.active(style)
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color { a: 0.1, ..self.text }
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        self.text
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        self.primary
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        iced::Color { a: 0.1, ..self.text }
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: color!(0x292C33).into(),
            border_radius: 6.0.into(),
            border_width: 1.0,
            border_color: color!(0x474B56),
            icon_color: color!(0xC9CCD3),
        }
    }
}
