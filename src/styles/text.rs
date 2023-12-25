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

use iced::color;
use iced::widget::text::{Appearance, StyleSheet};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Text {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
}

impl StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> Appearance {
        let color = match style {
            Text::Primary => color!(0xC9CCD3),
            Text::Secondary => color!(0x757983),
            Text::Success => color!(0x49914C),
            Text::Danger => color!(0xDF5658),
        };

        Appearance { color: Some(color) }
    }
}
