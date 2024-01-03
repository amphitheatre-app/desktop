// Copyright 2023 The Amphitheatre Authors.
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

use iced::alignment::{Alignment, Horizontal};
use iced::widget::{component, Column, Component, Text};
use iced::Length;

use crate::styles::constants::{FONT_SIZE_LARGE, FONT_SIZE_STANDARD, SPACING_NORMAL};

use super::{Container, Element, Renderer};

pub struct EmptyState {
    tagline: String,
    message: Option<String>,
}

pub fn empty(tagline: impl Into<String>) -> EmptyState {
    EmptyState::new(tagline, None::<String>)
}

impl EmptyState {
    pub fn new(tagline: impl Into<String>, message: impl Into<Option<String>>) -> Self {
        Self {
            tagline: tagline.into(),
            message: message.into(),
        }
    }
}

impl<Message> Component<Message, Renderer> for EmptyState {
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        let tagline = Text::new(&self.tagline)
            .size(FONT_SIZE_LARGE)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center);

        let message = if let Some(message) = &self.message {
            Text::new(message)
                .size(FONT_SIZE_STANDARD)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
        } else {
            Text::new("")
                .size(16)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
        };

        let content = Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(SPACING_NORMAL)
            .push(tagline)
            .push(message);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl<'a, Message> From<EmptyState> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(empty: EmptyState) -> Self {
        component(empty)
    }
}
