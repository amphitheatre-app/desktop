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
use iced::widget::Container;
use iced::{Alignment, Length, Subscription};
use iced_aw::graphics::icons::icon_to_char;
use iced_aw::{Icon, ICON_FONT};

use crate::theme;
use crate::widgets::{Button, Column, Element, Row, Scrollable, Text, TextInput};

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
    PlaybookSelected(Playbook),
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

                        // This will trigger a refresh of the playbooks.
                        self.update(Message::RefreshPlaybooks);
                    }
                }
            }
            Message::PlaybookSelected(playbook) => {
                println!("Playbook selected: {:?}", playbook);
            }
        }
    }

    /// poll playbooks from the server every 5 seconds
    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(5)).map(|_| Message::RefreshPlaybooks)
    }

    pub fn view(&self) -> Element<Message> {
        let playbooks = self.playbooks.iter().fold(
            Column::new().width(Length::Fill).height(Length::Fill),
            |column, playbook| {
                let item = item(&playbook.title, &playbook.description);
                // let height = item.as_widget().height();
                column.push(
                    Button::new(item)
                        .style(theme::Button::Element)
                        .width(Length::Fill)
                        .on_press(Message::PlaybookSelected(playbook.clone())),
                )
            },
        );

        let content = Column::new()
            .push(self.context_selector())
            .push(self.omnibox())
            .push(Scrollable::new(
                Container::new(playbooks).width(Length::Fill).height(Length::Shrink),
            ))
            .padding(16)
            .spacing(16);

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
                        Text::new(icon_to_char(Icon::ChevronExpand).to_string())
                            .font(ICON_FONT)
                            .size(16.0),
                    )
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .style(theme::Button::Element)
            .on_press(Message::ContextSelectorPressed),
        )
        .into()
    }

    fn omnibox(&self) -> Element<Message> {
        Row::new()
            .push(TextInput::new("Search", &self.query).on_input(Message::TextInputChanged))
            .push(
                Button::new(
                    Text::new(icon_to_char(Icon::Plus).to_string())
                        .font(ICON_FONT)
                        .width(Length::Fixed(20.0)),
                )
                .on_press(Message::CreateButtonPressed),
            )
            .spacing(4)
            .into()
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

fn item<'a>(title: impl ToString, _description: impl ToString) -> Element<'a, Message> {
    let icon = Text::new(icon_to_char(Icon::Box).to_string())
        .font(ICON_FONT)
        .size(14.0);

    // let content = Column::new()
    //     .push(Text::new(title.to_string()))
    //     .push(
    //         Text::new(_description.to_string())
    //             .style(theme::Text::Secondary)
    //             .size(14),
    //     )
    //     .width(Length::Fill);

    let content = Row::new()
        .push(icon)
        .push(Text::new(title.to_string()))
        .align_items(Alignment::Start)
        .spacing(8);
    Container::new(content).into()
}
