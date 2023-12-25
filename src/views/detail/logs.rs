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

use std::io::{BufRead, Cursor};

use iced::alignment::Horizontal;
use iced::{Alignment, Length};
use iced_aw::TabLabel;

use crate::widgets::tabs::Tab;
use crate::widgets::{Column, Container, Element, Scrollable, Text};

#[derive(Clone, Debug)]
pub enum Message {}

#[derive(Default)]
pub struct Logs {
    buffer: Vec<String>,
}

const LOGS: &[u8] = include_bytes!("../../../assets/test.access.log");

impl Logs {
    pub fn new() -> Self {
        Self {
            buffer: Cursor::new(LOGS).lines().map(|line| line.unwrap()).collect(),
        }
    }

    pub fn update(&mut self, _message: Message) {}
}

impl Tab for Logs {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Logs")
    }

    fn label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content = self
            .buffer
            .iter()
            .fold(Column::new().spacing(4).align_items(Alignment::Start), |column, log| {
                let text = Text::new(log).size(14).horizontal_alignment(Horizontal::Left);
                column.push(Container::new(text).width(Length::Fill))
            });

        Scrollable::new(content).into()
    }
}
