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
use iced_aw::style::card::Appearance;
use iced_aw::style::card::StyleSheet;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        let color = iced::Color {
            r: self.background.r + 0.1,
            g: self.background.r + 0.1,
            b: self.background.r + 0.1,
            a: self.background.a + 0.1,
        };

        Appearance {
            background: color.into(),
            head_background: color.into(),
            foot_background: color.into(),
            border_width: 0.0,
            close_color: self.text,
            ..Appearance::default()
        }
    }
}
