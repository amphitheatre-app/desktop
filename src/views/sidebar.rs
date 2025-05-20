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

use std::time::Duration;
use tracing::{debug, error};

use iced::{Alignment, Length, Subscription, Task};
use iced_fonts::{Bootstrap as Icon, BOOTSTRAP_FONT as ICON_FONT};

use crate::cmd::config::switch_context;
use crate::cmd::playbook::{compose, refresh_playbooks};
use crate::context::Context;
use crate::errors::Result;
use crate::styles::{self, constants::*};
use crate::utils::connection_status::ConnectionStatus;
use crate::widgets::context_switcher::{self, *};
use crate::widgets::{Button, Column, Container, Element, Row, Scrollable, Text, TextInput};
use amp_common::resource::PlaybookSpec;

use super::composer::{self, Composer};

pub struct Sidebar {
    ctx: Context,
    query: String,
    playbooks: Vec<PlaybookSpec>,
    status: ConnectionStatus,
    show_modal: bool,
    selected_playbook: Option<PlaybookSpec>,
    switcher: ContextSwitcher,
    composer: Composer,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum Message {
    Initializing,
    RefreshPlaybooks(Result<()>),
    PlaybooksLoaded(Result<Vec<PlaybookSpec>>),

    CreateButtonPressed,
    TextInputChanged(String),
    PlaybookSelected(Option<Result<PlaybookSpec>>),

    Switcher(context_switcher::Message),
    Composer(composer::Message),
}

impl Sidebar {
    pub fn new(ctx: Context) -> Self {
        let config = ctx.configuration();
        let context = config.context.as_ref().unwrap();
        let (name, cluster) = context.current().unwrap_or_default();
        let status = ConnectionStatus::default();

        let switcher = ContextSwitcher::new(name, cluster.title, context.clusters().clone(), status.clone());

        Self {
            ctx,
            query: String::new(),
            playbooks: vec![],
            status,
            show_modal: false,
            selected_playbook: None,
            switcher,
            composer: Composer::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Initializing => {
                self.selected_playbook = None;
                return Task::perform(refresh_playbooks(self.ctx.clone()), Message::PlaybooksLoaded);
            }
            Message::RefreshPlaybooks(arg) => match arg {
                Ok(_) => return Task::perform(refresh_playbooks(self.ctx.clone()), Message::PlaybooksLoaded),
                Err(e) => {
                    error!("Failed to refresh playbooks: {}", e);
                }
            },
            Message::PlaybooksLoaded(result) => match result {
                Ok(playbooks) => {
                    debug!(
                        "Playbooks loaded: {:?}",
                        playbooks.iter().map(|p| p.id.clone()).collect::<Vec<_>>()
                    );
                    self.playbooks = playbooks;
                    self.status = ConnectionStatus::Connected;
                }
                Err(e) => {
                    error!("Failed to load playbooks: {}", e);
                    self.playbooks = vec![];
                    self.status = ConnectionStatus::Disconnected;
                }
            },

            Message::CreateButtonPressed => self.show_modal = true,
            Message::TextInputChanged(query) => self.query = query,
            Message::PlaybookSelected(result) => {
                if let Some(result) = result {
                    match result {
                        Ok(playbook) => {
                            debug!("Playbook selected: {:?}", playbook);
                            self.selected_playbook = Some(playbook);
                        }
                        Err(e) => {
                            error!("Failed to select playbook: {}", e);
                            self.selected_playbook = None;
                        }
                    }
                }
            }
            Message::Switcher(message) => {
                let action = self.switcher.update(message);

                match action {
                    Action::None => {}
                    Action::Switch(name) => {
                        debug!("The current context was changed: {:?}", name);
                        self.selected_playbook = None;
                        return Task::perform(switch_context(self.ctx.clone(), name), Message::RefreshPlaybooks);
                    }
                }
            }
            Message::Composer(message) => {
                let action = self.composer.update(message);

                match action {
                    composer::Action::None => {}
                    composer::Action::Submit(form) => {
                        debug!("Form submitted: {:?}", form);
                        self.show_modal = false;

                        return Task::perform(
                            compose(self.ctx.clone(), form.title, form.description, form.preface, form.live),
                            |p| Message::PlaybookSelected(Some(p)),
                        );
                    }
                    composer::Action::Cancel => {
                        self.show_modal = false;
                        self.composer.reset();
                    }
                }
            }
        };

        Task::none()
    }

    /// poll playbooks from the server every 5 seconds
    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(5)).map(|_| Message::RefreshPlaybooks(Ok(())))
    }

    pub fn view(&self) -> Element<Message> {
        let selected_playbook_id = self.selected_playbook.as_ref().map(|p| &p.id);
        let playbooks = Scrollable::new(self.playbooks.iter().fold(
            Column::new().width(Length::Fill).spacing(SPACING_NORMAL),
            |column, playbook| {
                let active = Some(&playbook.id) == selected_playbook_id;
                column.push(playbook_item(playbook, active))
            },
        ))
        .width(Length::Fill)
        .height(Length::Shrink);

        let content = Column::new()
            .push(self.switcher.view().map(Message::Switcher))
            .push(self.omnibox())
            .push(playbooks)
            .padding(16)
            .spacing(16)
            .height(Length::Fill);

        Container::new(content)
            .style(styles::container::sidebar)
            .width(SIDEBAR_WIDTH)
            .height(Length::Fill)
            .into()
    }
}

impl Sidebar {
    fn omnibox(&self) -> Element<Message> {
        Row::new()
            .push(TextInput::new("Search", &self.query).on_input(Message::TextInputChanged))
            .push(self.button())
            .spacing(4)
            .into()
    }

    fn button(&self) -> Element<Message> {
        let underlay = Button::new(
            Text::new(Icon::Plus.to_string())
                .font(ICON_FONT)
                .size(ICON_FONT_SIZE_TINY)
                .width(Length::Fixed(20.0)),
        )
        .on_press(Message::CreateButtonPressed);

        let _overlay = if self.show_modal {
            Some(self.composer.view().map(Message::Composer))
        } else {
            None
        };

        underlay.into()

        // Modal::new(underlay, overlay)
        //     .backdrop(Message::CloseComposeModal)
        //     .on_esc(Message::CloseComposeModal)
        //     .into()
    }
}

fn playbook_item(playbook: &PlaybookSpec, active: bool) -> Element<Message> {
    let icon = Text::new(Icon::Box.to_string())
        .font(ICON_FONT)
        .size(ICON_FONT_SIZE_SIDEBAR);

    let content = Row::new()
        .push(icon)
        .push(Text::new(&playbook.title))
        .align_y(Alignment::Center)
        .spacing(8);

    Button::new(content)
        .style(if active {
            styles::button::primary
        } else {
            styles::button::text
        })
        .width(Length::Fill)
        .on_press(Message::PlaybookSelected(Some(Ok(playbook.clone()))))
        .into()
}
