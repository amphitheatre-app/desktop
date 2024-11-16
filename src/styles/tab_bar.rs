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

use iced::Background;
use iced_aw::style::{
    status::{Status, StyleFn},
    tab_bar::{Catalog, Style},
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

pub fn primary(theme: &Theme, status: Status) -> Style {
    let mut base = Style::default();
    let palette = theme.extended_palette();

    base.text_color = palette.background.base.text;

    match status {
        Status::Disabled => {
            base.tab_label_background = Background::Color(palette.background.strong.color);
        }
        Status::Hovered => {
            base.tab_label_background = Background::Color(palette.primary.strong.color);
        }
        _ => {
            base.tab_label_background = Background::Color(palette.primary.base.color);
        }
    }

    base
}
