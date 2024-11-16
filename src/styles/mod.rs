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
    color,
    theme::{
        palette::{self, Extended},
        Palette,
    },
};

pub mod application;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod constants;
pub mod container;
pub mod menu;
pub mod rule;
pub mod scrollable;
pub mod tab_bar;
pub mod text;
pub mod text_input;

/// The custom theme for the application.
#[derive(Clone, Debug)]
pub struct Theme {
    palette: Palette,
    extended: palette::Extended,
}

impl Theme {
    /// Returns the [`Palette`] of the [`Theme`].
    pub fn palette(&self) -> &Palette {
        &self.palette
    }

    /// Returns the [`palette::Extended`] of the [`Theme`].
    pub fn extended_palette(&self) -> &palette::Extended {
        &self.extended
    }
}

impl Default for Theme {
    fn default() -> Self {
        let palette = Palette {
            background: color!(0x1B1F22),
            text: color!(0xDFE2E6),
            primary: color!(0x79B8FF),
            success: color!(0x34CC57),
            danger: color!(0x79B8FF),
        };
        let extended = Extended::generate(palette);

        Self { palette, extended }
    }
}
