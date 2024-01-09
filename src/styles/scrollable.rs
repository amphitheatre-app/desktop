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

use iced::widget::scrollable::{Scrollbar, Scroller, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Scrollbar {
        Scrollbar {
            background: Default::default(),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Default::default(),
            scroller: Scroller {
                color: Default::default(),
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Default::default(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_mouse_over_scrollbar: bool) -> Scrollbar {
        self.active(style)
    }
}
