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

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use iced::{Alignment, Length, Subscription, Task};
use iced_aw::TabLabel;

use amp_common::resource::{CharacterSpec, PlaybookSpec};

use crate::cmd::actor::refresh_actor_stats;
use crate::context::Context;
use crate::errors::Result;
use crate::widgets::tabs::Tab;
use crate::widgets::{Column, Container, Element, Row, Rule, Text};

#[derive(Clone, Debug)]
pub enum Message {
    Initializing,
    StatsLoaded(Result<HashMap<String, String>>),
}

pub struct Stats {
    ctx: Context,
    data: HashMap<String, String>,
    playbook: Arc<PlaybookSpec>,
    character: Arc<CharacterSpec>,
}

impl Stats {
    pub fn new(ctx: Context, playbook: Arc<PlaybookSpec>, character: Arc<CharacterSpec>) -> Self {
        Self {
            ctx,
            data: Default::default(),
            playbook,
            character,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Initializing => {
                let pid = self.playbook.id.clone();
                let name = self.character.meta.name.clone();
                return Task::perform(refresh_actor_stats(self.ctx.clone(), pid, name), Message::StatsLoaded);
            }
            Message::StatsLoaded(data) => self.data = data.unwrap_or_default(),
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(5)).map(|_| Message::Initializing)
    }

    pub fn view(&self) -> Element<'_, Message> {
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
    fn cell(&self, label: impl ToString, value: impl ToString) -> Element<'_, Message> {
        Container::new(
            Column::new()
                .push(Text::new(value.to_string()).size(26))
                .push(Text::new(label.to_string()).size(16))
                .align_x(Alignment::Center)
                .spacing(32),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center(Length::Fill)
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
    fn view(&self) -> Element<'_, Self::Message> {
        self.view()
    }
}
