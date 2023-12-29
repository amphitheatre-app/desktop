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

use iced_aw::style::card::Appearance;
use iced_aw::style::card::StyleSheet;

use super::constants::ACCENT;
use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: ACCENT.into(),
            head_background: ACCENT.into(),
            foot_background: ACCENT.into(),
            border_width: 0.0,
            ..Appearance::default()
        }
    }
}
