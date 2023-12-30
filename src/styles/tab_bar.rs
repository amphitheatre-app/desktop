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

use iced_aw::tab_bar::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, is_active: bool) -> Appearance {
        Appearance {
            tab_label_background: iced::Color::TRANSPARENT.into(),
            tab_label_border_color: iced::Color::TRANSPARENT,
            text_color: if is_active {
                self.text
            } else {
                iced::Color { a: 0.1, ..self.text }
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> Appearance {
        self.active(style, is_active)
    }
}
