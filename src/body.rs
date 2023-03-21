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

use iced::widget::{horizontal_space, Rule};
use iced::{Alignment, Length};
use iced_aw::native::IconText;
use iced_aw::Icon;

use crate::components::tabs::Tab;
use crate::detail::envrionment::{self, Envrionment};
use crate::detail::logs::{self, Logs};
use crate::detail::resources::{self, Resources};
use crate::detail::stats::{self, Stats};
use crate::theme;
use crate::widget::{Button, Column, Container, Element, Row, Tabs, Text};

#[derive(Default)]
pub struct Body {
    active_tab: usize,
    logs: Logs,
    resources: Resources,
    envrionment: Envrionment,
    stats: Stats,
}

#[derive(Clone, Debug)]
pub enum Message {
    ButtonPressed,
    TabSelected(usize),

    Logs(logs::Message),
    Resources(resources::Message),
    Envrionment(envrionment::Message),
    Stats(stats::Message),
}

impl Body {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            logs: Logs::new(),
            resources: Resources::new(),
            envrionment: Envrionment::new(),
            stats: Stats::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {}
            Message::TabSelected(tab) => self.active_tab = tab,
            Message::Logs(message) => self.logs.update(message),
            Message::Resources(message) => self.resources.update(message),
            Message::Envrionment(message) => self.envrionment.update(message),
            Message::Stats(message) => self.stats.update(message),
        }
    }

    pub fn view(&self) -> Element<Message> {
        Container::new(
            Column::new()
                .push(self.toolbar())
                .push(Rule::horizontal(1))
                .push(self.tabs()),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    /// toolbar
    fn toolbar(&self) -> Element<Message> {
        Container::new(
            Row::new()
                .push(self.header())
                .push(horizontal_space(Length::Fill))
                .push(self.actions())
                .width(Length::Fill)
                .align_items(Alignment::Center),
        )
        .padding(16)
        .into()
    }

    fn header(&self) -> Element<Message> {
        let title = Column::new()
            .push(Text::new("Clean code linters"))
            .push(Text::new("Running").size(14).style(theme::Text::Success));

        Row::new()
            .push(IconText::new(Icon::List).width(Length::Fixed(20.0)))
            .push(title)
            .align_items(Alignment::Center)
            .spacing(8)
            .into()
    }

    fn actions(&self) -> Element<Message> {
        Row::new()
            .push(
                Button::new(IconText::new(Icon::Play).width(Length::Fixed(20.0)))
                    .on_press(Message::ButtonPressed),
            )
            .push(
                Button::new(IconText::new(Icon::Stop).width(Length::Fixed(20.0)))
                    .on_press(Message::ButtonPressed),
            )
            .push(
                Button::new(IconText::new(Icon::ArrowRepeat).width(Length::Fixed(20.0)))
                    .on_press(Message::ButtonPressed),
            )
            .push(
                Button::new(IconText::new(Icon::X).width(Length::Fixed(20.0)))
                    .on_press(Message::ButtonPressed),
            )
            .align_items(Alignment::Center)
            .spacing(4)
            .into()
    }

    fn tabs(&self) -> Element<Message> {
        Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.logs.label(), self.logs.view().map(Message::Logs))
            .push(
                self.resources.label(),
                self.resources.view().map(Message::Resources),
            )
            .push(
                self.envrionment.label(),
                self.envrionment.view().map(Message::Envrionment),
            )
            .push(self.stats.label(), self.stats.view().map(Message::Stats))
            .height(Length::Shrink)
            .into()
    }
}
