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

use iced::Border;
use iced_aw::style::menu_bar::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        let color = iced::Color {
            r: self.background.r + 0.1,
            g: self.background.r + 0.1,
            b: self.background.r + 0.1,
            a: self.background.a + 0.1,
        };

        Appearance {
            bar_background: color.into(),
            bar_border: Border {
                width: 1.0,
                color: iced::Color { a: 0.1, ..self.text },
                radius: 6.0.into(),
            },
            bar_background_expand: [8; 4].into(),
            path: iced::Color::TRANSPARENT.into(),
            ..Default::default()
        }
    }
}
