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

use iced::{
    theme::palette,
    widget::checkbox::{Catalog, Status, Style, StyleFn},
    Background, Border, Color,
};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// A primary checkbox; denoting a main toggle.
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    match status {
        Status::Active { is_checked } => styled(
            palette.primary.strong.text,
            palette.background.base,
            palette.primary.strong,
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            palette.primary.strong.text,
            palette.background.weak,
            palette.primary.base,
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            palette.primary.strong.text,
            palette.background.weak,
            palette.background.strong,
            is_checked,
        ),
    }
}

fn styled(icon_color: Color, base: palette::Pair, accent: palette::Pair, is_checked: bool) -> Style {
    Style {
        background: Background::Color(if is_checked { accent.color } else { base.color }),
        icon_color,
        border: Border {
            radius: 0.0.into(),
            width: 1.0,
            color: accent.color,
        },
        text_color: None,
    }
}
