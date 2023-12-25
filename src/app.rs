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

use amp_common::config::Configuration;
use iced::{executor, Application, Command, Length, Subscription};
use iced_aw::split;

use crate::config::{self, ConfigurationError};
use crate::styles::Theme;
use crate::views::body::{self, Body};
use crate::views::sidebar::{self, Sidebar};
use crate::widgets::{Container, Element, Split, Text};

#[derive(Clone, Debug)]
pub enum Message {
    ConfigurationMessage(Result<Configuration, ConfigurationError>),

    SidebarMessage(sidebar::Message),
    BodyMessage(body::Message),
    SplitResized(u16),
}

pub struct App {
    sidebar: Sidebar,
    body: Body,
    divider_position: Option<u16>,
    configuration: Option<Configuration>,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let app = Self {
            sidebar: Sidebar::new(),
            body: Body::new(),
            divider_position: Some(220),
            configuration: None,
        };

        let command = Command::perform(config::load(), Message::ConfigurationMessage);

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Amphitheatre Desktop")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::ConfigurationMessage(Ok(configuration)) => {
                self.sidebar
                    .update(sidebar::Message::ContextLoaded(configuration.context.clone()));
                self.configuration = Some(configuration);
            }
            Message::ConfigurationMessage(Err(err)) => {
                eprintln!("Could not load configuration: {}", err);
            }
            Message::SidebarMessage(sidebar::Message::PlaybookSelected(playbook)) => {
                self.body.update(body::Message::PlaybookSelected(playbook));
            }
            Message::SidebarMessage(message) => self.sidebar.update(message),
            Message::BodyMessage(message) => self.body.update(message),
            Message::SplitResized(position) => self.divider_position = Some(position),
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            self.sidebar.subscription().map(Message::SidebarMessage),
            self.body.subscription().map(Message::BodyMessage),
        ])
    }

    fn view(&self) -> Element<'_, Self::Message> {
        if self.configuration.is_none() {
            return empty(Text::new("Loading..."));
        }

        Split::new(
            self.sidebar.view().map(Message::SidebarMessage),
            self.body.view().map(Message::BodyMessage),
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
