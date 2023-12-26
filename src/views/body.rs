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

use std::sync::Arc;

use amp_client::playbooks::Playbook;
use iced::widget::{horizontal_space, Rule};
use iced::{Alignment, Length, Subscription};
use iced_aw::graphics::icons::icon_to_char;
use iced_aw::{Icon, ICON_FONT};

use crate::context::Context;
use crate::styles;
use crate::views::detail::inspect::{self, Information};
use crate::views::detail::logs::{self, Logs};
use crate::views::detail::stats::{self, Stats};
use crate::widgets::tabs::Tab;
use crate::widgets::{Button, Column, Container, Element, Row, Tabs, Text};

// #[derive(Default)]
pub struct Body {
    playbook: Playbook,
    active_tab: TabId,
    logs: Logs,
    info: Information,
    stats: Stats,
}

#[derive(Clone, Debug)]
pub enum Message {
    ButtonPressed,
    TabSelected(TabId),

    Logs(logs::Message),
    Info(inspect::Message),
    Stats(stats::Message),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabId {
    #[default]
    Logs,
    Info,
    Stats,
}

impl Body {
    pub fn new(ctx: Arc<Context>, playbook: Playbook) -> Self {
        Self {
            playbook: playbook.clone(),
            active_tab: TabId::default(),
            logs: Logs::new(ctx.clone(), playbook.clone()),
            info: Information::new(ctx.clone(), playbook.clone()),
            stats: Stats::new(ctx.clone(), playbook.clone()),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {}
            Message::TabSelected(tab) => self.active_tab = tab,
            Message::Logs(message) => self.logs.update(message),
            Message::Info(message) => self.info.update(message),
            Message::Stats(message) => self.stats.update(message),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.logs.subscription().map(Message::Logs),
            self.info.subscription().map(Message::Info),
            self.stats.subscription().map(Message::Stats),
        ])
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
}

impl Body {
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
            .push(Text::new(&self.playbook.title))
            .push(Text::new("Running").size(14).style(styles::Text::Success));

        Row::new()
            .push(
                Text::new(icon_to_char(Icon::List).to_string())
                    .font(ICON_FONT)
                    .width(Length::Fixed(20.0)),
            )
            .push(title)
            .align_items(Alignment::Center)
            .spacing(8)
            .into()
    }

    fn actions(&self) -> Element<Message> {
        Row::new()
            .push(
                Button::new(
                    Text::new(icon_to_char(Icon::Play).to_string())
                        .font(ICON_FONT)
                        .width(Length::Fixed(20.0)),
                )
                .on_press(Message::ButtonPressed),
            )
            .push(
                Button::new(
                    Text::new(icon_to_char(Icon::Stop).to_string())
                        .font(ICON_FONT)
                        .width(Length::Fixed(20.0)),
                )
                .on_press(Message::ButtonPressed),
            )
            .push(
                Button::new(
                    Text::new(icon_to_char(Icon::ArrowRepeat).to_string())
                        .font(ICON_FONT)
                        .width(Length::Fixed(20.0)),
                )
                .on_press(Message::ButtonPressed),
            )
            .push(
                Button::new(
                    Text::new(icon_to_char(Icon::X).to_string())
                        .font(ICON_FONT)
                        .width(Length::Fixed(20.0)),
                )
                .on_press(Message::ButtonPressed),
            )
            .align_items(Alignment::Center)
            .spacing(4)
            .into()
    }

    fn tabs(&self) -> Element<Message> {
        Tabs::new(Message::TabSelected)
            .push(TabId::Logs, self.logs.label(), self.logs.content().map(Message::Logs))
            .push(TabId::Info, self.info.label(), self.info.content().map(Message::Info))
            .push(
                TabId::Stats,
                self.stats.label(),
                self.stats.content().map(Message::Stats),
            )
            .set_active_tab(&self.active_tab)
            .tab_label_padding(8.0)
            .height(Length::Shrink)
            .into()
    }
}
