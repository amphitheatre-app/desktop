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

use iced::widget::container::{transparent, Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn sidebar(theme: &Theme) -> Style {
    let palette = theme.palette();
    let mut color = palette.background;
    color.r += 0.05;
    color.g += 0.05;
    color.b += 0.05;

    Style {
        background: Some(color.into()),
        ..Style::default()
    }
}

pub fn toolbar(theme: &Theme) -> Style {
    let palette = theme.palette();
    let mut color = palette.background;
    color.r += 0.03;
    color.g += 0.03;
    color.b += 0.03;

    Style {
        background: Some(color.into()),
        ..Style::default()
    }
}
