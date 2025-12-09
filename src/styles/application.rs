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

use iced::theme::{Base, Mode, Style};

use super::Theme;

impl Base for Theme {
    fn default(_preference: Mode) -> Self {
        <Theme as Default>::default()
    }

    fn mode(&self) -> Mode {
        Mode::Dark
    }

    fn base(&self) -> Style {
        let palette = self.extended_palette();

        Style {
            background_color: palette.background.base.color,
            text_color: palette.background.base.text,
        }
    }

    fn palette(&self) -> Option<iced::theme::Palette> {
        Some(*self.palette())
    }

    fn name(&self) -> &str {
        "Amphitheatre"
    }
}
