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

use std::time::Duration;

use amp_client::client::Client;
use amp_client::playbooks::Playbook;
use amp_common::config::{Cluster, ContextConfiguration};
use iced::widget::Rule;
use iced::{color, Alignment, Length, Subscription};
use iced_aw::native::IconText;
use iced_aw::Icon;

use crate::theme;
use crate::widget::{Button, Column, Container, Element, Row, Scrollable, Text, TextInput};

#[derive(Debug)]
pub struct Sidebar {
    query: String,
    playbooks: Vec<Playbook>,
    contexts: Option<ContextConfiguration>,
    // current_context
    context: Cluster,
    state: State,
}

#[derive(Clone, Debug)]
pub enum Message {
    ContextSelectorPressed,
    CreateButtonPressed,
    TextInputChanged(String),
    RefreshPlaybooks,
    ContextLoaded(Option<ContextConfiguration>),
}

#[derive(Clone, Debug)]
pub enum State {
    Connecting,
    Connected,
    Disconnected,
}

// impl std::fmt::Display for State
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Connecting => write!(f, "Connecting..."),
            State::Connected => write!(f, "Connected"),
            State::Disconnected => write!(f, "Disconnected. Retrying..."),
        }
    }
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            playbooks: vec![],
            contexts: None,
            context: Cluster {
                title: "Unknown".to_string(),
                ..Cluster::default()
            },
            state: State::Connecting,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ContextSelectorPressed => {}
            Message::CreateButtonPressed => {}
            Message::TextInputChanged(query) => self.query = query,
            Message::RefreshPlaybooks => {
                let url = &format!("{}/v1", &self.context.server);
                let client = Client::new(url, self.context.token.clone());

                match client.playbooks().list(None) {
                    Ok(playbooks) => {
                        self.playbooks = playbooks;
                        self.state = State::Connected;
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch playbooks, error: {}", e);
                        self.state = State::Disconnected;
                    }
                }
            }
            Message::ContextLoaded(contexts) => {
                self.contexts = contexts;
                if let Some(contexts) = &self.contexts {
                    if let Some(context) = contexts.current() {
                        self.context = context.clone();
                    }
                }
            }
        }
    }

    /// poll playbooks from the server every 5 seconds
    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(5)).map(|_| Message::RefreshPlaybooks)
    }

    pub fn view(&self) -> Element<Message> {
        let mut v: Vec<Element<Message>> = vec![];

        if self.playbooks.is_empty() {
            v.push(empty(
                Text::new("No playbooks").size(16).style(theme::Text::Secondary),
            ));
        } else if self.playbooks.len() > 10 {
            v.push(self.omnibox());
        } else {
            let playbooks = self.playbooks.iter().fold(Column::new(), |column, playbook| {
                column.push(item(&playbook.title, &playbook.description))
            });
            v.push(Scrollable::new(playbooks).into());
        }

        let items = Column::with_children(v).spacing(16).width(Length::Fill);
        let content = Column::new().push(self.context_selector()).push(items);

        Container::new(content).height(Length::Fill).into()
    }

    fn context_selector(&self) -> Element<Message> {
        let style = match self.state {
            State::Connecting => theme::Text::Secondary,
            State::Connected => theme::Text::Success,
            State::Disconnected => theme::Text::Danger,
        };
        let text = self.state.to_string();
        let state = Row::new()
            .push(Text::new("•").size(14).style(style))
            .push(Text::new(text).size(14).style(theme::Text::Secondary))
            .align_items(Alignment::Center);

        let heading = Column::new()
            .push(Text::new(self.context.title.to_string()))
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

fn empty<'a, T>(content: T) -> Element<'a, Message>
where
    T: Into<Element<'a, Message>>,
{
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
