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

use iced::Color;
use iced_aw::style::{
    menu_bar::{Catalog, Style},
    status::{Status, StyleFn},
};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut color = palette.background.base.color;
    color.r += 0.05;
    color.g += 0.05;
    color.b += 0.05;

    Style {
        bar_background: Color::TRANSPARENT.into(),
        menu_background: color.into(),
        path: palette.primary.weak.color.into(),
        ..Default::default()
    }
}
