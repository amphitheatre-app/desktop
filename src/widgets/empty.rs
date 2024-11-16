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

use iced::advanced::text;
use iced::alignment::{Alignment, Horizontal};
use iced::Length;

use crate::widgets::{Column, Container, Element, Text};

use crate::styles::constants::*;

pub fn empty<'a, Message: 'a>(
    tagline: impl text::IntoFragment<'a>,
    message: Option<impl text::IntoFragment<'a>>,
) -> Element<'a, Message> {
    let tagline = Text::new(tagline)
        .size(FONT_SIZE_LARGE)
        .width(Length::Fill)
        .align_x(Horizontal::Center);

    let message = if let Some(message) = message {
        Text::new(message)
            .size(FONT_SIZE_STANDARD)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
    } else {
        Text::new("").size(16).width(Length::Fill).align_x(Horizontal::Center)
    };

    let content = Column::new()
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(SPACING_NORMAL)
        .push(tagline)
        .push(message);

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center(Length::Fill)
        .into()
}
