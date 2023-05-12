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
use iced::widget::{horizontal_space, Rule};
use iced::{Alignment, Length, Subscription};
use iced_aw::native::IconText;
use iced_aw::Icon;

use crate::components::tabs::Tab;
use crate::detail::inspect::{self, Information};
use crate::detail::logs::{self, Logs};
use crate::detail::stats::{self, Stats};
use crate::theme;
use crate::widget::{Button, Column, Container, Element, Row, Tabs, Text};

#[derive(Default)]
pub struct Body {
    playbook: Option<Playbook>,
    active_tab: usize,
    logs: Logs,
    info: Information,
    stats: Stats,
}

#[derive(Clone, Debug)]
pub enum Message {
    ButtonPressed,
    TabSelected(usize),

    Logs(logs::Message),
    Information(inspect::Message),
    Stats(stats::Message),
    PlaybookSelected(Playbook),
}

impl Body {
    pub fn new() -> Self {
        Self {
            playbook: None,
            active_tab: 0,
            logs: Logs::new(),
            info: Information::new(),
            stats: Stats::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {}
            Message::TabSelected(tab) => self.active_tab = tab,
            Message::Logs(message) => self.logs.update(message),
            Message::Information(message) => self.info.update(message),
            Message::Stats(message) => self.stats.update(message),
            Message::PlaybookSelected(playbook) => self.playbook = Some(playbook),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn view(&self) -> Element<Message> {
        if self.playbook.is_none() {
            return empty(Text::new("No playbook selected"));
        }

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
        let title = &self.playbook.as_ref().unwrap().title;

        let title = Column::new()
            .push(Text::new(title))
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
            .push(self.info.label(), self.info.view().map(Message::Information))
            .push(self.stats.label(), self.stats.view().map(Message::Stats))
            .tab_label_padding(8.0)
            .height(Length::Shrink)
            .into()
    }
}

fn empty<'a, T>(content: T) -> Element<'a, Message>
where
    T: Into<Element<'a, Message>>,
{
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
