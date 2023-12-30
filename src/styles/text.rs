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

use iced::widget::text::{Appearance, StyleSheet};
use iced::Color;

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Text {
    #[default]
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
}

impl StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> Appearance {
        let color = match style {
            Text::Default => self.text,
            Text::Primary => self.primary,
            Text::Secondary => Color { a: 0.25, ..self.text },
            Text::Success => self.success,
            Text::Danger => self.danger,
        };

        Appearance { color: Some(color) }
    }
}
