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

use amp_client::playbooks::Playbook;
use iced::widget::{Row, Rule};
use iced::{Alignment, Length, Subscription};
use iced_aw::TabLabel;

use crate::widgets::tabs::Tab;
use crate::widgets::{Column, Container, Element, Text};

#[derive(Clone, Debug)]
pub enum Message {}

pub struct Stats {
    playbook: Playbook,
}

impl Stats {
    pub fn new(playbook: Playbook) -> Self {
        Self { playbook }
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

impl Tab for Stats {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Stats")
    }

    fn label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        println!("The playbook is #{:?}", self.playbook.id);

        let content = Column::new()
            .push(
                Row::new()
                    .push(self.cell("0.92%", "CPU USAGE"))
                    .push(Rule::vertical(1))
                    .push(self.cell("118.3 MB", "MEMORY USAGE"))
                    .width(Length::Fill)
                    .height(Length::FillPortion(5)),
            )
            .push(Rule::horizontal(1))
            .push(
                Row::new()
                    .push(self.cell("62.1 MB / 216 kB", "DISK READ/WRITE"))
                    .push(Rule::vertical(1))
                    .push(self.cell("0 Bytes / 0 Bytes", "NETWORK IO"))
                    .width(Length::Fill)
                    .height(Length::FillPortion(5)),
            )
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(content).into()
    }
}

impl Stats {
    fn cell(&self, value: impl ToString, label: impl ToString) -> Element<Message> {
        Container::new(
            Column::new()
                .push(Text::new(value.to_string()).size(26))
                .push(Text::new(label.to_string()).size(16))
                .align_items(Alignment::Center)
                .spacing(32),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
