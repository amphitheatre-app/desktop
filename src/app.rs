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

use amp_common::config::{Cluster, Configuration};
use iced::{executor, Application, Command, Length};
use iced_aw::split;

use crate::body::{self, Body};
use crate::configuration::{self, ConfigurationError};
use crate::sidebar::{self, Sidebar};
use crate::theme::Theme;
use crate::widget::{Element, Split};

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
    configuration: Configuration,
    context: Option<Cluster>,
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
            configuration: Configuration::default(),
            context: None,
        };

        let command = Command::perform(configuration::load(), Message::ConfigurationMessage);

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Amphitheatre Desktop")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::ConfigurationMessage(Ok(configuration)) => {
                self.configuration = configuration;
                if let Some(context) = &self.configuration.context {
                    self.context = context.current().cloned();
                }
            }
            Message::ConfigurationMessage(Err(err)) => {
                eprintln!("Could not load configuration: {}", err);
            }
            Message::SidebarMessage(message) => self.sidebar.update(message),
            Message::BodyMessage(message) => self.body.update(message),
            Message::SplitResized(position) => self.divider_position = Some(position),
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Split::new(
            self.sidebar.view(&self.context).map(Message::SidebarMessage),
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
