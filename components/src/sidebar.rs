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

use iced::widget::{column, container, row, rule, scrollable, text, Container, Rule, Scrollable, Text};
use iced::{color, theme, Alignment, Element, Length, Theme};
use icon::{icon, Icon};
const CONTEXT_NAME: &str = "Amphitheatre Local";
const DISCONNECTED: &str = "Disconnected. Retrying...";

#[derive(Debug, Default)]
pub struct Sidebar {}

pub enum Message {}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<Message> {
        // Context selector
        let context = row![
            column![
                text(CONTEXT_NAME),
                row![
                    icon(Icon::Dot).style(color!(0xDF5658)).width(10),
                    Text::new(DISCONNECTED).size(14).style(color!(0xA7A9AD))
                ]
            ]
            .width(Length::Fill),
            icon(Icon::ChevronExpand).size(16).style(color!(0xA7A9AD))
        ]
        .padding([0, 0, 20, 0])
        .align_items(Alignment::Center)
        .width(Length::Fill);

        // Playbook
        let playbooks = column![
            playbook(
                "Clean code linters",
                "Make sure your code matches your style guide with these essential code linters."
            ),
            playbook(
                "Open journalism",
                "See how publications and data-driven journalists use open source to power their ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
            playbook(
                "Design essentials",
                "This collection of design libraries are the best on the web, and will complete your ..."
            ),
        ];

        let content = column![context, Scrollable::new(playbooks).style(ScrollableStyle)];
        Container::new(content)
            .style(theme::Container::Custom(Box::new(SidebarStyle)))
            .width(240.0)
            .height(Length::Fill)
            .padding([20, 12])
            .into()
    }
}

fn playbook<'a>(title: impl ToString, description: impl ToString) -> Element<'a, Message> {
    let title = text(title);
    let desc = text(description).size(14).style(color!(0xA7A9AD));
    let icon = icon(Icon::Box).width(24).height(26).size(24);

    column![
        row![
            Container::new(icon).width(36).center_y().height(Length::Fill),
            column![title, desc].width(Length::Fill).height(Length::Fill)
        ]
        .padding([8, 0])
        .width(Length::Fill)
        .height(64),
        Rule::horizontal(1).style(RuleStyle)
    ]
    .into()
}

struct SidebarStyle;

impl container::StyleSheet for SidebarStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: color!(0x30343D).into(),
            ..Default::default()
        }
    }
}

struct ScrollableStyle;

impl scrollable::StyleSheet for ScrollableStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Default::default(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
            scroller: scrollable::Scroller {
                color: Default::default(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Default::default(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
            scroller: scrollable::Scroller {
                color: Default::default(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
            },
        }
    }
}

impl Into<iced::theme::Scrollable> for ScrollableStyle {
    fn into(self) -> iced::theme::Scrollable {
        iced::theme::Scrollable::custom(self)
    }
}

struct RuleStyle;

impl rule::StyleSheet for RuleStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: color!(0x474B56),
            width: 1,
            radius: 0.0,
            fill_mode: rule::FillMode::Full,
        }
    }
}

impl Into<iced::theme::Rule> for RuleStyle {
    fn into(self) -> iced::theme::Rule {
        iced::theme::Rule::Custom(Box::new(RuleStyle))
    }
}
