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

use std::hash::Hash;
use std::sync::Arc;

use futures::StreamExt;
use iced::widget::scrollable;
use iced::{Font, Length, Subscription, Task};
use iced_aw::TabLabel;
use iced_futures::subscription::{from_recipe, Hasher};
use iced_futures::{subscription, BoxStream};
use reqwest_eventsource::Event;

use amp_common::resource::{CharacterSpec, PlaybookSpec};

use crate::context::Context;
use crate::styles::constants::{FONT_SIZE_SMALL, SPACING_SMALL};
use crate::widgets::tabs::Tab;
use crate::widgets::{Column, Element, Scrollable, Text};

#[derive(Clone, Debug)]
pub enum Message {
    Received(String),
    Errored(String),
}

pub struct Logs {
    ctx: Arc<Context>,
    playbook: Arc<PlaybookSpec>,
    character: Arc<CharacterSpec>,
    messages: Vec<String>,
    scrollable_id: scrollable::Id,
}

impl Logs {
    pub fn new(ctx: Arc<Context>, playbook: Arc<PlaybookSpec>, character: Arc<CharacterSpec>) -> Self {
        Self {
            ctx,
            playbook,
            character,
            messages: vec![],
            scrollable_id: scrollable::Id::unique(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Received(message) => {
                self.messages.push(message);
                scrollable::snap_to(self.scrollable_id.clone(), scrollable::RelativeOffset::END)
            }
            Message::Errored(message) => {
                self.messages.push(message);
                scrollable::snap_to(self.scrollable_id.clone(), scrollable::RelativeOffset::END)
            }
        }
    }

    // Tail the log stream from the server
    pub fn subscription(&self) -> Subscription<Message> {
        from_recipe(Receiver::new(
            self.ctx.clone(),
            &self.playbook.id,
            &self.character.meta.name,
        ))
    }

    pub fn view(&self) -> Element<Message> {
        let content = Column::with_children(
            self.messages
                .iter()
                .map(|message| Text::new(message).size(FONT_SIZE_SMALL).font(Font::MONOSPACE).into()),
        )
        .width(Length::Fill)
        .spacing(SPACING_SMALL);

        Scrollable::new(content)
            .id(self.scrollable_id.clone())
            .height(Length::Fill)
            .into()
    }
}

impl Tab for Logs {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Logs")
    }

    fn label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    #[inline]
    fn view(&self) -> Element<Self::Message> {
        self.view()
    }
}

struct Receiver {
    es: reqwest_eventsource::EventSource,
    pid: String,
    name: String,
}

impl Receiver {
    pub fn new(ctx: Arc<Context>, pid: &str, name: &str) -> Self {
        Self {
            es: ctx.client.read().unwrap().actors().logs(pid, name),
            pid: String::from(pid),
            name: String::from(name),
        }
    }
}

const OPEN_STREAM_MESSAGE: &str = "Receiving the log stream from the server...";

impl subscription::Recipe for Receiver {
    type Output = Message;

    fn hash(&self, state: &mut Hasher) {
        std::any::TypeId::of::<Self>().hash(state);
        self.pid.hash(state);
        self.name.hash(state);
    }

    fn stream(self: Box<Self>, _: subscription::EventStream) -> BoxStream<Self::Output> {
        futures::stream::unfold(self.es, |mut es| async {
            let event = es.next().await;
            match event {
                Some(Ok(Event::Open)) => Some((Message::Received(String::from(OPEN_STREAM_MESSAGE)), es)),
                Some(Ok(Event::Message(message))) => Some((Message::Received(message.data), es)),
                Some(Err(e)) => Some((Message::Errored(e.to_string()), es)),
                _ => Some((Message::Errored(format!("{:#?}", event)), es)),
            }
        })
        .boxed()
    }
}
