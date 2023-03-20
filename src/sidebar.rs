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

use iced::widget::Rule;
use iced::{color, Alignment, Length};
use iced_aw::native::IconText;
use iced_aw::Icon;

use crate::theme;
use crate::widget::{Button, Column, Container, Element, Row, Scrollable, Text, TextInput};

const CONTEXT_NAME: &str = "Amphitheatre Local";
const DISCONNECTED: &str = "Disconnected. Retrying...";

#[derive(Debug, Default)]
pub struct Sidebar {
    query: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    ButtonPressed,
    TextInputChanged(String),
}

impl Sidebar {
    pub fn new() -> Self {
        Self { query: String::new() }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {}
            Message::TextInputChanged(query) => self.query = query,
        }
    }

    fn context_selector(&self) -> Element<Message> {
        let state = Row::new()
            .push(Text::new("•").size(14).style(theme::Text::Danger))
            .push(Text::new(DISCONNECTED).size(14).style(theme::Text::Secondary))
            .align_items(Alignment::Center);

        let heading = Column::new()
            .push(Text::new(CONTEXT_NAME))
            .push(state)
            .width(Length::Fill);

        Container::new(
            Row::new()
                .push(heading)
                .push(
                    IconText::new(Icon::ChevronExpand)
                        .size(16.0)
                        .color(color!(0x474B56)),
                )
                .align_items(Alignment::Center)
                .width(Length::Fill),
        )
        .padding(16)
        .into()
    }

    pub fn view(&self) -> Element<Message> {
        // Context selector

        let context = self.context_selector();

        // Playbook
        let playbooks = Column::new()
            .push(playbook(
                "Clean code linters",
                "Make sure your code matches your style guide with these essential code linters.",
            ))
            .push(playbook(
                "Open journalism",
                "See how publications and data-driven journalists use open source to power their ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ))
            .push(playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ...",
            ));

        let omnibox = Row::new()
            .push(TextInput::new("Search", &self.query, Message::TextInputChanged))
            .push(
                Button::new(IconText::new(Icon::Plus).width(Length::Fixed(20.0)))
                    .on_press(Message::ButtonPressed),
            )
            .padding([0, 0, 16, 0])
            .spacing(4);

        let content = Column::new().push(context).push(
            Column::new()
                .push(omnibox)
                .push(Scrollable::new(playbooks))
                .padding(16),
        );

        Container::new(content).height(Length::Fill).into()
    }
}

fn playbook<'a>(title: impl ToString, description: impl ToString) -> Element<'a, Message> {
    let icon = Container::new(
        IconText::new(Icon::Box)
            .width(Length::Fixed(24.0))
            .height(Length::Fixed(26.0))
            .size(24.0),
    )
    .width(36)
    .center_y()
    .height(Length::Fill);

    let content = Column::new()
        .push(Text::new(title.to_string()))
        .push(
            Text::new(description.to_string())
                .size(14)
                .style(theme::Text::Secondary),
        )
        .width(Length::Fill)
        .height(Length::Fill);

    Column::new()
        .push(
            Row::new()
                .push(icon)
                .push(content)
                .padding([8, 0])
                .width(Length::Fill)
                .height(64),
        )
        .push(Rule::horizontal(1))
        .into()
}
