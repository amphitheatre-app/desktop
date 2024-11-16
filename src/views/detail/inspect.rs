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

use iced::{Length, Subscription, Task};
use iced_aw::TabLabel;

use amp_common::resource::{CharacterSpec, PlaybookSpec};

use crate::cmd::actor::refresh_actor_info;
use crate::context::Context;
use crate::errors::Result;
use crate::styles::constants::SPACING_NORMAL;
use crate::widgets::empty::empty;
use crate::widgets::tabs::Tab;
use crate::widgets::{Column, Element, Row, Rule, Scrollable, Text};

#[derive(Clone, Debug)]
pub enum Message {
    Initializing,
    InfoLoaded(Result<HashMap<String, HashMap<String, String>>>),
}

pub struct Information {
    ctx: Arc<Context>,
    data: HashMap<String, HashMap<String, String>>,
    playbook: Arc<PlaybookSpec>,
    character: Arc<CharacterSpec>,
}

impl Information {
    pub fn new(ctx: Arc<Context>, playbook: Arc<PlaybookSpec>, character: Arc<CharacterSpec>) -> Self {
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
                return Task::perform(refresh_actor_info(self.ctx.clone(), pid, name), Message::InfoLoaded);
            }
            Message::InfoLoaded(data) => self.data = data.unwrap_or_default(),
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(5)).map(|_| Message::Initializing)
    }

    pub fn view(&self) -> Element<Message> {
        if self.data.is_empty() {
            return empty("No information available", None::<String>);
        }

        let mut children = vec![];

        for (group, fields) in &self.data {
            children.push(Text::new(group.to_ascii_uppercase()).size(24).into());
            for (key, value) in fields {
                children.push(
                    Column::new()
                        .push(
                            Row::new()
                                .push(Text::new(key).size(16).width(Length::FillPortion(4)))
                                .push(Text::new(value).size(14).width(Length::FillPortion(6)))
                                .width(Length::Fill),
                        )
                        .push(Rule::horizontal(1))
                        .width(Length::Fill)
                        .spacing(SPACING_NORMAL)
                        .into(),
                );
            }
        }

        let content = Column::with_children(children)
            .padding(16)
            .spacing(SPACING_NORMAL)
            .width(Length::Fill);
        Scrollable::new(content).into()
    }
}

impl Tab for Information {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Inspect")
    }

    fn label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    #[inline]
    fn view(&self) -> Element<Self::Message> {
        self.view()
    }
}
