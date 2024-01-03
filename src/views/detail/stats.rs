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

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::cmd::actor::refresh_actor_stats;
use crate::context::Context;
use crate::errors::Result;
use crate::widgets::tabs::Tab;
use crate::widgets::{Column, Container, Element, Text};
use amp_client::playbooks::Playbook;
use iced::widget::{Row, Rule};
use iced::{Alignment, Command, Length, Subscription};
use iced_aw::TabLabel;

#[derive(Clone, Debug)]
pub enum Message {
    Initializing,
    StatsLoaded(Result<HashMap<String, String>>),
}

pub struct Stats {
    ctx: Arc<Context>,
    data: HashMap<String, String>,
    playbook: Playbook,
}

impl Stats {
    pub fn new(ctx: Arc<Context>, playbook: Playbook) -> Self {
        Self {
            ctx,
            data: Default::default(),
            playbook,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Initializing => {
                let pid = self.playbook.id.to_string();
                let name = "amp-example-go".to_string();
                return Command::perform(refresh_actor_stats(self.ctx.clone(), pid, name), Message::StatsLoaded);
            }
            Message::StatsLoaded(data) => self.data = data.unwrap_or_default(),
        }

        Command::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(5)).map(|_| Message::Initializing)
    }

    pub fn view(&self) -> Element<Message> {
        let content = Column::new()
            .push(
                Row::new()
                    .push(self.cell("CPU USAGE", self.cpu_usage_value()))
                    .push(Rule::vertical(1))
                    .push(self.cell("MEMORY USAGE", self.memory_usage_value()))
                    .width(Length::Fill)
                    .height(Length::FillPortion(5)),
            )
            .push(Rule::horizontal(1))
            .push(
                Row::new()
                    .push(self.cell("DISK READ/WRITE", "62.1 MB / 216 kB"))
                    .push(Rule::vertical(1))
                    .push(self.cell("NETWORK IO", "0 Bytes / 0 Bytes"))
                    .width(Length::Fill)
                    .height(Length::FillPortion(5)),
            )
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(content).into()
    }
}

impl Stats {
    fn cell(&self, label: impl ToString, value: impl ToString) -> Element<Message> {
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

    fn cpu_usage_value(&self) -> String {
        self.data.get("CPU USAGE").unwrap_or(&String::from("0")).to_string()
    }

    fn memory_usage_value(&self) -> String {
        self.data.get("MEMORY USAGE").unwrap_or(&String::from("0")).to_string()
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

    #[inline]
    fn view(&self) -> Element<Self::Message> {
        self.view()
    }
}
