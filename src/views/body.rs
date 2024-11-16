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

use std::sync::Arc;

use iced::widget::horizontal_space;
use iced::{Alignment, Length, Subscription, Task};
use iced_fonts::{Bootstrap as Icon, BOOTSTRAP_FONT as ICON_FONT};

use amp_common::resource::{CharacterSpec, PlaybookSpec};

use crate::context::Context;
use crate::styles::{self, constants::*};
use crate::views::detail::inspect::{self, Information};
use crate::views::detail::logs::{self, Logs};
use crate::views::detail::stats::{self, Stats};
use crate::widgets::character_switcher::{self, *};
use crate::widgets::tabs::Tab;
use crate::widgets::{Button, Column, Container, Element, Row, Rule, Tabs, Text};

// #[derive(Default)]
pub struct Body {
    playbook: Arc<PlaybookSpec>,
    character: Arc<CharacterSpec>,
    switcher: CharacterSwitcher,
    active_tab: TabId,
    logs: Logs,
    info: Information,
    stats: Stats,
}

#[derive(Clone, Debug)]
pub enum Message {
    Initializing,

    CloseButtonPressed(Arc<PlaybookSpec>),
    TabSelected(TabId),

    Logs(logs::Message),
    Info(inspect::Message),
    Stats(stats::Message),

    Switcher(character_switcher::Message),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabId {
    #[default]
    Logs,
    Info,
    Stats,
}

impl Body {
    pub fn new(ctx: Arc<Context>, playbook: Arc<PlaybookSpec>, character: Arc<CharacterSpec>) -> Self {
        Self {
            playbook: playbook.clone(),
            character: character.clone(),
            switcher: CharacterSwitcher::new(playbook.clone(), character.clone()),
            active_tab: TabId::default(),
            logs: Logs::new(ctx.clone(), playbook.clone(), character.clone()),
            info: Information::new(ctx.clone(), playbook.clone(), character.clone()),
            stats: Stats::new(ctx.clone(), playbook.clone(), character.clone()),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Initializing => {
                return Task::batch(vec![
                    Task::perform(async {}, |_| Message::Info(inspect::Message::Initializing)),
                    Task::perform(async {}, |_| Message::Stats(stats::Message::Initializing)),
                ]);
            }
            Message::CloseButtonPressed(_) => {}
            Message::TabSelected(tab) => self.active_tab = tab,
            Message::Logs(message) => return self.logs.update(message).map(Message::Logs),
            Message::Info(message) => return self.info.update(message).map(Message::Info),
            Message::Stats(message) => return self.stats.update(message).map(Message::Stats),
            Message::Switcher(message) => {
                let action = self.switcher.update(message);

                match action {
                    Action::None => {}
                    Action::Switch(character) => self.character = character,
                };
            }
        }
        Task::none()
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
        .width(Length::Shrink)
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
                .push(horizontal_space())
                // .push(self.actions())
                .width(Length::Fill)
                .align_y(Alignment::Center),
        )
        .style(styles::container::toolbar)
        .padding(16)
        .into()
    }

    fn header(&self) -> Element<Message> {
        let mut items = vec![];
        if let Some(characters) = &self.playbook.characters {
            if characters.len() > 1 {
                items.push(self.switcher.view().map(Message::Switcher));
            }
        }

        items.push(
            Column::new()
                .push(Text::new(&self.character.meta.name))
                .push(
                    Text::new("Running")
                        .size(FONT_SIZE_SMALLER)
                        .style(styles::text::success),
                )
                .into(),
        );

        Row::with_children(items).align_y(Alignment::Center).spacing(8).into()
    }

    #[allow(dead_code)]
    fn actions(&self) -> Element<Message> {
        let button = |icon: Icon, on_press| {
            Button::new(Text::new(icon.to_string()).font(ICON_FONT).size(ICON_FONT_SIZE_TOOLBAR))
                .style(styles::button::text)
                .on_press(on_press)
        };

        Row::new()
            // .push(button(Icon::Play, Message::ButtonPressed))
            // .push(button(Icon::Stop, Message::ButtonPressed))
            // .push(button(Icon::ArrowRepeat, Message::ButtonPressed))
            .push(button(Icon::X, Message::CloseButtonPressed(self.playbook.clone())))
            .align_y(Alignment::Center)
            .spacing(SPACING_SMALL)
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
