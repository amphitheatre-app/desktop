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

use std::sync::Arc;

use amp_common::resource::{CharacterSpec, PlaybookSpec};
use iced::widget::{horizontal_space, Rule};
use iced::{Alignment, Command, Length, Subscription};
use iced_aw::{core::icons::bootstrap::Bootstrap as Icon, BOOTSTRAP_FONT as ICON_FONT};

use crate::context::Context;
use crate::styles;
use crate::styles::constants::{FONT_SIZE_SMALLER, ICON_FONT_SIZE_TOOLBAR, SPACING_NORMAL, SPACING_SMALL};
use crate::widgets::empty::empty;
use crate::widgets::lists::CharacterItem;
use crate::widgets::{Button, Column, Container, Element, Row, Scrollable, Text};

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
    pub fn new(_ctx: Arc<Context>, playbook: PlaybookSpec) -> Self {
        Self {
            playbook: playbook.clone(),
            selected_character: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Initializing => {}
            Message::CloseButtonPressed(_) => {}
            Message::CharacterSelected(character) => {
                self.selected_character = Some(*character);
            }
        }
        Command::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn view(&self) -> Element<Message> {
        let content: Element<Message>;
        if let Some(character) = &self.playbook.characters {
            content = character
                .iter()
                .fold(
                    Column::new()
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .spacing(SPACING_NORMAL),
                    |column, character| {
                        column.push(
                            CharacterItem::new(character.clone())
                                .active(
                                    self.selected_character
                                        .as_ref()
                                        .is_some_and(|c| c.meta.name == character.meta.name),
                                )
                                .on_press(|p| Message::CharacterSelected(Box::new(p))),
                        )
                    },
                )
                .into();
        } else {
            content = empty("No characters").into();
        }

        Container::new(
            Column::new()
                .push(self.toolbar())
                .push(Rule::horizontal(1))
                .push(Scrollable::new(
                    Container::new(content).width(Length::Fill).height(Length::Shrink),
                )),
        )
        .width(Length::Fill)
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
                .align_items(Alignment::Center),
        )
        .style(styles::Container::Toolbar)
        .padding(16)
        .into()
    }

    fn header(&self) -> Element<Message> {
        Row::new()
            .push(
                Column::new().push(Text::new(&self.playbook.title)).push(
                    Text::new("Running")
                        .size(FONT_SIZE_SMALLER)
                        .style(styles::Text::Success),
                ),
            )
            .align_items(Alignment::Center)
            .spacing(8)
            .into()
    }

    fn actions(&self) -> Element<Message> {
        let button = |icon: Icon, on_press| {
            Button::new(Text::new(icon.to_string()).font(ICON_FONT).size(ICON_FONT_SIZE_TOOLBAR))
                .style(styles::Button::Icon)
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
            .align_items(Alignment::Center)
            .spacing(SPACING_SMALL)
            .into()
    }
}
