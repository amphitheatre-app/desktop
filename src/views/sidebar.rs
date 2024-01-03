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

use iced_aw::{Icon, ICON_FONT};

use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error};

use amp_client::playbooks::Playbook;
use iced::alignment::Horizontal;
use iced::widget::Container;
use iced::{Command, Length, Subscription};

use crate::cmd::config::switch_context;
use crate::cmd::playbook::refresh_playbooks;
use crate::context::Context;
use crate::errors::Result;
use crate::styles::{self, constants::*};
use crate::utils::connection_status::ConnectionStatus;
use crate::widgets::lists::SidebarPlaybookItem;
use crate::widgets::*;

use super::compose::{self, Compose};

#[derive(Debug)]
pub struct Sidebar {
    ctx: Arc<Context>,
    query: String,
    playbooks: Vec<Playbook>,
    status: ConnectionStatus,
    show_modal: bool,
    compose_form: compose::Form,
    selected_playbook: Option<Playbook>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new(Arc::new(Context::default()))
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Initializing,
    RefreshPlaybooks(Result<()>),
    PlaybooksLoaded(Result<Vec<Playbook>>),

    ContextChanged(String),
    CreateButtonPressed,
    TextInputChanged(String),
    PlaybookSelected(Playbook),

    CloseComposeModal,
    ComposeFormChanged(compose::Form),
    ComposeFormSubmit,
}

impl Sidebar {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self {
            ctx,
            query: String::new(),
            playbooks: vec![],
            status: ConnectionStatus::default(),
            show_modal: false,
            compose_form: compose::Form::default(),
            selected_playbook: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Initializing => {
                return Command::perform(refresh_playbooks(self.ctx.clone()), Message::PlaybooksLoaded);
            }
            Message::RefreshPlaybooks(arg) => match arg {
                Ok(_) => return Command::perform(refresh_playbooks(self.ctx.clone()), Message::PlaybooksLoaded),
                Err(e) => {
                    error!("Failed to refresh playbooks: {}", e);
                }
            },
            Message::PlaybooksLoaded(playbooks) => {
                self.playbooks = playbooks.unwrap_or_default();
                self.status = ConnectionStatus::Connected;
            }
            Message::ContextChanged(name) => {
                debug!("The current context was changed: {:?}", name);
                return Command::perform(switch_context(self.ctx.clone(), name), Message::RefreshPlaybooks);
            }
            Message::CreateButtonPressed => self.show_modal = true,
            Message::TextInputChanged(query) => self.query = query,
            Message::PlaybookSelected(playbook) => {
                debug!("Playbook selected: {:?}", playbook);
                self.selected_playbook = Some(playbook);
            }
            Message::CloseComposeModal => {
                self.show_modal = false;
                self.compose_form = compose::Form::default();
            }
            Message::ComposeFormChanged(form) => self.compose_form = form,
            Message::ComposeFormSubmit => {
                println!("Form submitted: {:?}", self.compose_form);
                self.show_modal = false;
                self.compose_form = compose::Form::default();
            }
        };

        Command::none()
    }

    /// poll playbooks from the server every 5 seconds
    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(5)).map(|_| Message::RefreshPlaybooks(Ok(())))
    }

    pub fn view(&self) -> Element<Message> {
        let playbooks = self.playbooks.iter().fold(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(SPACING_NORMAL),
            |column, playbook| {
                column.push(
                    SidebarPlaybookItem::new(playbook.clone())
                        .active(self.selected_playbook.as_ref().is_some_and(|p| p.id == playbook.id))
                        .on_press(Message::PlaybookSelected),
                )
            },
        );

        let (name, cluster) = self.ctx.context().unwrap_or_default();
        let config = self.ctx.configuration.read().unwrap();
        let context = config.context.as_ref().unwrap();

        let context_switcher =
            ContextSwitcher::new(name, cluster.title, context.clusters().clone(), self.status.clone())
                .on_change(Message::ContextChanged);

        let content = Column::new()
            .push(context_switcher)
            .push(self.omnibox())
            .push(Scrollable::new(
                Container::new(playbooks).width(Length::Fill).height(Length::Shrink),
            ))
            .padding(16)
            .spacing(16);

        Container::new(content)
            .style(styles::Container::Sidebar)
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
                .horizontal_alignment(Horizontal::Center)
                .font(ICON_FONT)
                .size(ICON_FONT_SIZE_TINY)
                .width(Length::Fixed(20.0)),
        )
        .on_press(Message::CreateButtonPressed);

        let overlay = if self.show_modal {
            Some(Compose::new(
                self.compose_form.clone(),
                Message::ComposeFormChanged,
                Message::CloseComposeModal,
                Message::ComposeFormSubmit,
            ))
        } else {
            None
        };

        Modal::new(underlay, overlay)
            .backdrop(Message::CloseComposeModal)
            .on_esc(Message::CloseComposeModal)
            .into()
    }
}
