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

use super::Theme;

use iced::widget::checkbox::{Appearance, StyleSheet};

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, is_checked: bool) -> Appearance {
        Appearance {
            background: iced::Color::TRANSPARENT.into(),
            icon_color: self.text,
            border_radius: 0.0.into(),
            border_width: 1.0,
            border_color: if is_checked {
                self.primary
            } else {
                iced::Color { a: 0.1, ..self.text }
            },
            text_color: Some(self.text),
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> Appearance {
        self.active(style, is_checked)
    }
}
