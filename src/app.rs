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

use std::sync::Arc;

use iced::{executor, Application, Command, Length, Subscription};
use iced_aw::split;

use crate::context::Context;
use crate::styles::Theme;
use crate::views::body::{self, Body};
use crate::views::sidebar::{self, Sidebar};
use crate::widgets::empty::empty;
use crate::widgets::{Element, Split};

#[derive(Clone, Debug)]
pub enum Message {
    // Messages from the sub views.
    SidebarMessage(sidebar::Message),
    BodyMessage(body::Message),

    // Messages from self actions.
    SplitResized(u16),
}

pub struct App {
    ctx: Arc<Context>,
    sidebar: Sidebar,
    body: Option<Body>,
    divider_position: Option<u16>,
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
            divider_position: Some(220),
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Amphitheatre Desktop")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::SidebarMessage(sidebar::Message::PlaybookSelected(playbook)) => {
                self.body = Some(Body::new(self.ctx.clone(), playbook.clone()));
            }
            Message::SidebarMessage(message) => self.sidebar.update(message),
            Message::BodyMessage(message) => {
                if let Some(body) = &mut self.body {
                    body.update(message);
                }
            }
            Message::SplitResized(position) => self.divider_position = Some(position),
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
        ])
    }

    fn view(&self) -> Element<Self::Message> {
        let first = self.sidebar.view().map(Message::SidebarMessage);
        let second = self
            .body
            .as_ref()
            .map(|body| body.view().map(Message::BodyMessage))
            .unwrap_or(empty("No playbook selected").into());

        Split::new(
            first,
            second,
            self.divider_position,
            split::Axis::Vertical,
            Message::SplitResized,
        )
        .min_size_first(200)
        .min_size_second(790)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(1.0)
        .into()
    }
}
