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

use iced::widget::horizontal_space;
use iced::{Alignment, Length, Subscription, Task};
use iced_fonts::{Bootstrap as Icon, BOOTSTRAP_FONT as ICON_FONT};

use amp_common::resource::{CharacterSpec, PlaybookSpec};

use crate::context::Context;
use crate::styles::{self, constants::*};
use crate::widgets::empty::empty;
use crate::widgets::{Button, Column, Container, Element, Row, Rule, Scrollable, Text};

// #[derive(Default)]
pub struct Cast {
    playbook: PlaybookSpec,
    selected_character: Option<CharacterSpec>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Initializing,

    CloseButtonPressed(Box<PlaybookSpec>),
    CharacterSelected(Box<CharacterSpec>),
}

impl Cast {
    pub fn new(_ctx: Context, playbook: PlaybookSpec) -> Self {
        Self {
            playbook: playbook.clone(),
            selected_character: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Initializing => {}
            Message::CloseButtonPressed(_) => {}
            Message::CharacterSelected(character) => {
                self.selected_character = Some(*character);
            }
        }
        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn view(&self) -> Element<Message> {
        let selected_character_name = self.selected_character.as_ref().map(|c| &c.meta.name);

        let content: Element<Message>;
        if let Some(character) = &self.playbook.characters {
            content = Scrollable::new(
                character.iter().fold(
                    Column::new()
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .spacing(SPACING_NORMAL),
                    |column, character: &CharacterSpec| {
                        let active = Some(&character.meta.name) == selected_character_name;
                        column.push(character_item(character, active))
                    },
                ),
            )
            .width(Length::Fill)
            .height(Length::Shrink)
            .into();
        } else {
            content = empty("No characters", None::<String>);
        };

        Container::new(
            Column::new()
                .push(self.toolbar())
                .push(Rule::horizontal(1))
                .push(content),
        )
        .width(Length::Shrink)
        .height(Length::Fill)
        .into()
    }
}

impl Cast {
    /// toolbar
    fn toolbar(&self) -> Element<Message> {
        Container::new(
            Row::new()
                .push(self.header())
                .push(horizontal_space())
                .push(self.actions())
                .width(Length::Fill)
                .align_y(Alignment::Center),
        )
        .style(styles::container::toolbar)
        .padding(16)
        .into()
    }

    fn header(&self) -> Element<Message> {
        Row::new()
            .push(
                Column::new().push(Text::new(&self.playbook.title)).push(
                    Text::new("Running")
                        .size(FONT_SIZE_SMALLER)
                        .style(styles::text::success),
                ),
            )
            .align_y(Alignment::Center)
            .spacing(8)
            .into()
    }

    fn actions(&self) -> Element<Message> {
        let button = |icon: Icon, on_press| {
            Button::new(Text::new(icon.to_string()).font(ICON_FONT).size(ICON_FONT_SIZE_TOOLBAR))
                .style(styles::button::text)
                .on_press(on_press)
        };

        Row::new()
            // .push(button(Icon::Play, Message::ButtonPressed))
            // .push(button(Icon::Stop, Message::ButtonPressed))
            // .push(button(Icon::ArrowRepeat, Message::ButtonPressed))
            .push(button(
                Icon::X,
                Message::CloseButtonPressed(Box::new(self.playbook.clone())),
            ))
            .align_y(Alignment::Center)
            .spacing(SPACING_SMALL)
            .into()
    }
}

fn character_item(character: &CharacterSpec, active: bool) -> Element<Message> {
    let icon = Text::new(Icon::Box.to_string())
        .font(ICON_FONT)
        .size(ICON_FONT_SIZE_SIDEBAR);

    let content = Row::new()
        .push(icon)
        .push(Text::new(&character.meta.name))
        .align_y(Alignment::Center)
        .spacing(8);

    Button::new(content)
        .style(if active {
            styles::button::primary
        } else {
            styles::button::text
        })
        .width(Length::Fill)
        .on_press(Message::CharacterSelected(Box::new(character.clone())))
        .into()
}
