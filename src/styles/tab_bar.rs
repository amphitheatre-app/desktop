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

use iced::{color, Color};
use iced_aw::tab_bar::{Appearance, StyleSheet};

use super::{constants::SURFACE, Theme};

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, is_active: bool) -> Appearance {
        let mut appearance = Appearance::default();

        let text_color = if is_active { color!(0xC9CCD3) } else { color!(0x848993) };

        appearance.tab_label_background = SURFACE.into();
        appearance.tab_label_border_width = 0.0;
        appearance.tab_label_border_color = Color::TRANSPARENT;
        appearance.icon_color = text_color;
        appearance.text_color = text_color;

        appearance
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> Appearance {
        Appearance {
            icon_color: color!(0xC9CCD3),
            text_color: color!(0xC9CCD3),
            ..self.active(style, is_active)
        }
    }
}
