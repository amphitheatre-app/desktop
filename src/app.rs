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
use iced::{executor, Application, Command, Length, Subscription};
use iced_aw::{split, BOOTSTRAP_FONT_BYTES};
use tracing::debug;

use crate::cmd::playbook::close_playbook;
use crate::context::Context;
use crate::styles::constants::{SIDEBAR_WIDTH, WINDOW_MIN_WIDTH};
use crate::styles::Theme;
use crate::views::body::{self, Body};
use crate::views::cast::{self, Cast};
use crate::views::sidebar::{self, Sidebar};
use crate::widgets::empty::empty;
use crate::widgets::{Element, Split};

#[derive(Clone, Debug)]
pub enum Message {
    // Messages from the sub views.
    SidebarMessage(sidebar::Message),
    BodyMessage(body::Message),
    CastMessage(cast::Message),

    // Messages from self actions.
    SplitResized(u16),

    None,
}

pub struct App {
    ctx: Arc<Context>,
    sidebar: Sidebar,
    body: Option<Body>,
    cast: Option<Cast>,
    divider_position: Option<u16>,

    selected_playbook: Option<PlaybookSpec>,
    selected_character: Option<CharacterSpec>,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Arc<Context>;

    fn new(ctx: Self::Flags) -> (Self, Command<Message>) {
        let app = Self {
            ctx: ctx.clone(),
            sidebar: Sidebar::new(ctx.clone()),
            body: None,
            cast: None,
            divider_position: Some(220),
            selected_playbook: None,
            selected_character: None,
        };

        let commands = Command::batch(vec![
            iced::font::load(BOOTSTRAP_FONT_BYTES).map(|_| Message::None),
            Command::perform(async {}, |_| Message::SidebarMessage(sidebar::Message::Initializing)),
            Command::perform(async {}, |_| Message::BodyMessage(body::Message::Initializing)),
        ]);

        (app, commands)
    }

    fn title(&self) -> String {
        String::from("Amphitheatre Desktop")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::SidebarMessage(message) => {
                if let sidebar::Message::PlaybookSelected(result) = &message {
                    self.body = None;
                    self.cast = None;

                    if let Some(Ok(playbook)) = result {
                        self.selected_playbook = Some(playbook.clone());
                        if let Some(characters) = &playbook.characters {
                            debug!("characters: {:?}", characters);
                            if characters.len() > 1 {
                                self.cast = Some(Cast::new(self.ctx.clone(), playbook.clone()));
                            } else {
                                self.body = Some(Body::new(
                                    self.ctx.clone(),
                                    playbook.clone(),
                                    characters.first().unwrap().clone(),
                                ));
                            }
                        }
                    }
                }
                // Reset the body when the context changes.
                if let sidebar::Message::ContextChanged(_) = &message {
                    self.body = None;
                    self.cast = None;
                }
                return self.sidebar.update(message).map(Message::SidebarMessage);
            }
            Message::BodyMessage(message) => {
                if let body::Message::CloseButtonPressed(playbook) = message {
                    return Command::perform(close_playbook(self.ctx.clone(), playbook.id), |_| {
                        Message::SidebarMessage(sidebar::Message::PlaybookSelected(None))
                    });
                }

                if let body::Message::CharacterSelected(character) = &message {
                    self.selected_character = Some(*character.clone());

                    self.body = Some(Body::new(
                        self.ctx.clone(),
                        self.selected_playbook.clone().unwrap(),
                        *character.clone(),
                    ));
                }

                if let Some(body) = &mut self.body {
                    return body.update(message.clone()).map(Message::BodyMessage);
                }
            }
            Message::CastMessage(message) => {
                if let cast::Message::CloseButtonPressed(playbook) = message {
                    return Command::perform(close_playbook(self.ctx.clone(), playbook.id), |_| {
                        Message::SidebarMessage(sidebar::Message::PlaybookSelected(None))
                    });
                }

                if let cast::Message::CharacterSelected(character) = &message {
                    self.selected_character = Some(*character.clone());

                    self.body = Some(Body::new(
                        self.ctx.clone(),
                        self.selected_playbook.clone().unwrap(),
                        *character.clone(),
                    ));
                }

                if let Some(actors) = &mut self.cast {
                    return actors.update(message.clone()).map(Message::CastMessage);
                }
            }
            Message::SplitResized(position) => self.divider_position = Some(position),
            Message::None => {}
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
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

    fn view(&self) -> Element<Self::Message> {
        let first = self.sidebar.view().map(Message::SidebarMessage);

        // the second view is the actors view if playbook is selected,
        // and the actors of playbook is more than 1.
        // the second view is the body view if playbook is selected,
        // and the self.actor is selected.
        let second = if let Some(body) = &self.body {
            body.view().map(Message::BodyMessage)
        } else if let Some(actors) = &self.cast {
            actors.view().map(Message::CastMessage)
        } else {
            empty("No playbook selected").into()
        };

        Split::new(
            first,
            second,
            self.divider_position,
            split::Axis::Vertical,
            Message::SplitResized,
        )
        .min_size_first(SIDEBAR_WIDTH)
        .min_size_second(WINDOW_MIN_WIDTH as u16)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(1.0)
        .into()
    }
}
