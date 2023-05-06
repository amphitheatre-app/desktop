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

use amp_common::config::Cluster;
use iced::widget::Rule;
use iced::{color, Alignment, Length};
use iced_aw::native::IconText;
use iced_aw::Icon;

use crate::theme;
use crate::util::strings::generate_random_words_string;
use crate::widget::{Button, Column, Container, Element, Row, Scrollable, Text, TextInput};

const DISCONNECTED: &str = "Disconnected. Retrying...";

#[derive(Debug)]
pub struct Sidebar {
    query: String,
    playbooks: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
pub enum Message {
    ContextSelectorPressed,
    CreateButtonPressed,
    TextInputChanged(String),
}

impl Sidebar {
    pub fn new() -> Self {
        let playbooks = (0..10)
            .map(|_| {
                (
                    generate_random_words_string(2..3),
                    generate_random_words_string(8..16),
                )
            })
            .collect();

        Self {
            query: String::new(),
            playbooks,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ContextSelectorPressed => {}
            Message::CreateButtonPressed => {}
            Message::TextInputChanged(query) => self.query = query,
        }
    }

    pub fn view(&self, ctx: &Option<Cluster>) -> Element<Message> {
        let context = match ctx {
            Some(ctx) => self.context_selector(ctx),
            None => Text::new("No context selected").into(),
        };

        let playbooks = self.playbooks.iter().fold(Column::new(), |column, playbook| {
            column.push(item(&playbook.0, &playbook.1))
        });

        let content = Column::new().push(context).push(
            Column::new()
                .push(self.omnibox())
                .push(Scrollable::new(playbooks))
                .padding(16),
        );

        Container::new(content).height(Length::Fill).into()
    }

    fn context_selector(&self, ctx: &Cluster) -> Element<Message> {
        let state = Row::new()
            .push(Text::new("•").size(14).style(theme::Text::Danger))
            .push(Text::new(DISCONNECTED).size(14).style(theme::Text::Secondary))
            .align_items(Alignment::Center);

        let heading = Column::new()
            .push(Text::new(ctx.title.to_string()))
            .push(state)
            .width(Length::Fill);

        Container::new(
            Button::new(
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
            .style(theme::Button::Element)
            .on_press(Message::ContextSelectorPressed),
        )
        .padding(16)
        .into()
    }

    fn omnibox(&self) -> Element<Message> {
        Row::new()
            .push(TextInput::new("Search", &self.query, Message::TextInputChanged))
            .push(
                Button::new(IconText::new(Icon::Plus).width(Length::Fixed(20.0)))
                    .on_press(Message::CreateButtonPressed),
            )
            .padding([0, 0, 16, 0])
            .spacing(4)
            .into()
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

fn item<'a>(title: impl ToString, description: impl ToString) -> Element<'a, Message> {
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
