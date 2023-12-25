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

use iced::Color;
use iced_aw::split::{Appearance, StyleSheet};

use super::{constants::ACCENT, Theme};

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            first_background: Some(ACCENT.into()),
            divider_background: ACCENT.into(),
            divider_border_color: ACCENT,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            ..Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }

    fn dragged(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }
}
