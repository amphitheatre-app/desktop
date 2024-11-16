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

use iced::theme::palette;
use iced_aw::style::{
    card::{Catalog, Style},
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
    styled(theme, palette.primary.strong)
}

pub fn secondary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();
    styled(theme, palette.secondary.strong)
}

pub fn success(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();
    styled(theme, palette.success.strong)
}

pub fn danger(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();
    styled(theme, palette.danger.strong)
}

fn styled(theme: &Theme, pair: palette::Pair) -> Style {
    let palette = theme.extended_palette();
    let foreground = theme.palette();

    Style {
        border_color: pair.color,
        head_background: pair.color.into(),
        head_text_color: pair.text,
        close_color: pair.text,
        background: palette.background.base.color.into(),
        body_text_color: foreground.text,
        foot_text_color: foreground.text,
        ..Style::default()
    }
}
