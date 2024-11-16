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
    widget::{
        container,
        scrollable::{Catalog, Rail, Scroller, Status, Style, StyleFn},
    },
    Border,
};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`Scrollable`].
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let scrollbar = Rail {
        background: Some(palette.background.weak.color.into()),
        border: Border::default(),
        scroller: Scroller {
            color: palette.background.strong.color,
            border: Border::default(),
        },
    };

    match status {
        Status::Active => Style {
            container: container::Style::default(),
            vertical_rail: scrollbar,
            horizontal_rail: scrollbar,
            gap: None,
        },
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
        } => {
            let hovered_scrollbar = Rail {
                scroller: Scroller {
                    color: palette.primary.strong.color,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container::Style::default(),
                vertical_rail: if is_vertical_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
            }
        }
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
        } => {
            let dragged_scrollbar = Rail {
                scroller: Scroller {
                    color: palette.primary.base.color,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container::Style::default(),
                vertical_rail: if is_vertical_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
            }
        }
    }
}
