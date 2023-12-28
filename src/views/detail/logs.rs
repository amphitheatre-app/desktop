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

use std::hash::Hash;
use std::sync::Arc;

use amp_client::playbooks::Playbook;
use futures::StreamExt;
use iced::widget::scrollable;
use iced::{Command, Length, Subscription};
use iced_aw::TabLabel;
use iced_futures::{subscription, BoxStream};
use reqwest_eventsource::Event;

use crate::context::Context;
use crate::widgets::tabs::Tab;
use crate::widgets::{Column, Element, Scrollable};

#[derive(Clone, Debug)]
pub enum Message {
    Received(String),
    Errored(String),
}

pub struct Logs {
    ctx: Arc<Context>,
    playbook: Playbook,
    messages: Vec<String>,
    scrollable_id: scrollable::Id,
}

impl Logs {
    pub fn new(ctx: Arc<Context>, playbook: Playbook) -> Self {
        Self {
            ctx,
            playbook,
            messages: vec![],
            scrollable_id: scrollable::Id::unique(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
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
        Subscription::from_recipe(Receiver::new(
            self.ctx.clone(),
            &self.playbook.id,
            &String::from("amp-example-go"),
        ))
    }

    pub fn view(&self) -> Element<Message> {
        let content = Column::with_children(
            self.messages
                .iter()
                .cloned()
                .map(iced::widget::text)
                .map(Element::from)
                .collect(),
        )
        .width(Length::Fill)
        .spacing(10);

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
    ctx: Arc<Context>,
    pid: String,
    name: String,
}

impl Receiver {
    pub fn new(ctx: Arc<Context>, pid: &str, name: &str) -> Self {
        Self {
            ctx,
            pid: String::from(pid),
            name: String::from(name),
        }
    }
}

const OPEN_STREAM_MESSAGE: &str = "Receiving the log stream from the server...";

impl subscription::Recipe for Receiver {
    type Output = Message;

    fn hash(&self, state: &mut iced_futures::core::Hasher) {
        std::any::TypeId::of::<Self>().hash(state);
        self.pid.hash(state);
        self.name.hash(state);
    }

    fn stream(self: Box<Self>, _: subscription::EventStream) -> BoxStream<Self::Output> {
        futures::stream::unfold(self.ctx.client.actors().logs(&self.pid, &self.name), |mut es| async {
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
