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

use components::body::Body;
use components::sidebar::Sidebar;
use iced::widget::{Row, Rule};
use iced::{Element, Length, Sandbox};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SidebarMessage,
    BodyMessage,
}

pub struct App {
    sidebar: Sidebar,
    body: Body,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            sidebar: Default::default(),
            body: Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Amphitheatre Desktop")
    }

    fn update(&mut self, _message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(self.sidebar.view().map(|_| Message::SidebarMessage))
            .push(Rule::vertical(1))
            .push(self.body.view().map(|_| Message::BodyMessage))
            .into()
    }
}
