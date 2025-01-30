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

use iced::{Length, Subscription, Task};
use tracing::debug;

use crate::cmd::playbook::close_playbook;
use crate::context::Context;
use crate::styles::Theme;
use crate::views::body::{self, Body};
use crate::views::cast::{self, Cast};
use crate::views::sidebar::{self, Sidebar};
use crate::widgets::empty::empty;
use crate::widgets::{Container, Element, Row, Rule};

#[derive(Clone, Debug)]
pub enum Message {
    // Messages from the sub views.
    SidebarMessage(sidebar::Message),
    BodyMessage(body::Message),
    CastMessage(cast::Message),
}

pub struct App {
    theme: Theme,
    ctx: Context,

    sidebar: Sidebar,
    body: Option<Body>,
    cast: Option<Cast>,

    selected_playbook: Option<Arc<PlaybookSpec>>,
    selected_character: Option<Arc<CharacterSpec>>,
}

impl App {
    pub fn new(ctx: Context) -> (Self, Task<Message>) {
        let app = Self {
            theme: Theme::default(),
            ctx: ctx.clone(),
            sidebar: Sidebar::new(ctx.clone()),
            body: None,
            cast: None,
            selected_playbook: None,
            selected_character: None,
        };

        let commands = Task::batch(vec![
            Task::perform(async {}, |_| Message::SidebarMessage(sidebar::Message::Initializing)),
            Task::perform(async {}, |_| Message::BodyMessage(body::Message::Initializing)),
        ]);

        (app, commands)
    }

    pub fn title(&self) -> String {
        String::from("Amphitheatre Desktop")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SidebarMessage(message) => {
                if let sidebar::Message::PlaybookSelected(result) = &message {
                    self.body = None;
                    self.cast = None;

                    if let Some(Ok(playbook)) = result {
                        self.selected_playbook = Some(playbook.clone().into());
                        if let Some(characters) = &playbook.characters {
                            debug!("characters: {:?}", characters);
                            if characters.len() > 1 {
                                self.cast = Some(Cast::new(self.ctx.clone(), playbook.clone()));
                            } else {
                                self.body = Some(Body::new(
                                    self.ctx.clone(),
                                    playbook.clone().into(),
                                    characters.first().unwrap().clone().into(),
                                ));
                            }
                        }
                    }
                }
                // TODO: Reset the body when the context changes.
                // if let sidebar::Message::ContextChanged(_) = &message {
                //     self.body = None;
                //     self.cast = None;
                // }
                return self.sidebar.update(message).map(Message::SidebarMessage);
            }
            Message::BodyMessage(message) => {
                if let body::Message::CloseButtonPressed(playbook) = message {
                    return Task::perform(close_playbook(self.ctx.clone(), playbook.id.clone()), |_| {
                        Message::SidebarMessage(sidebar::Message::PlaybookSelected(None))
                    });
                }

                // TODO: Reset selected character
                // if let body::Message::CharacterSelected(character) = &message {
                //     self.selected_character = Some(*character.clone());

                //     self.body = Some(Body::new(
                //         self.ctx.clone(),
                //         self.selected_playbook.clone().unwrap(),
                //         *character.clone(),
                //     ));
                // }

                if let Some(body) = &mut self.body {
                    return body.update(message.clone()).map(Message::BodyMessage);
                }
            }
            Message::CastMessage(message) => {
                if let cast::Message::CloseButtonPressed(playbook) = message {
                    return Task::perform(close_playbook(self.ctx.clone(), playbook.id), |_| {
                        Message::SidebarMessage(sidebar::Message::PlaybookSelected(None))
                    });
                }

                if let cast::Message::CharacterSelected(character) = &message {
                    self.selected_character = Some((*character.clone()).into());

                    self.body = Some(Body::new(
                        self.ctx.clone(),
                        self.selected_playbook.clone().unwrap(),
                        (*character.clone()).into(),
                    ));
                }

                if let Some(actors) = &mut self.cast {
                    return actors.update(message.clone()).map(Message::CastMessage);
                }
            }
        }
        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.sidebar.subscription().map(Message::SidebarMessage),
            self.body
                .as_ref()
                .map(|body| body.subscription())
                .unwrap_or(Subscription::none())
                .map(Message::BodyMessage),
            self.cast
                .as_ref()
                .map(|actors| actors.subscription())
                .unwrap_or(Subscription::none())
                .map(Message::CastMessage),
        ])
    }

    pub fn view(&self) -> Element<Message> {
        Container::new(
            Row::new()
                .push(self.sidebar.view().map(Message::SidebarMessage))
                .push(Rule::vertical(1))
                .push(
                    Container::new(if let Some(body) = &self.body {
                        body.view().map(Message::BodyMessage)
                    } else if let Some(actors) = &self.cast {
                        actors.view().map(Message::CastMessage)
                    } else {
                        empty("No playbook selected", None::<String>)
                    })
                    .width(Length::Shrink)
                    .height(Length::Fill),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
