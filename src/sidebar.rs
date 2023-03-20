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

use iced::widget::{
    button, column, container, row, rule, scrollable, text, text_input, Button, Container, Rule, Scrollable,
    TextInput,
};
use iced::{color, Alignment, Color, Element, Length, Theme};
use icon::{icon, Icon};

const CONTEXT_NAME: &str = "Amphitheatre Local";
const DISCONNECTED: &str = "Disconnected. Retrying...";

#[derive(Debug, Default)]
pub struct Sidebar {}

#[derive(Clone, Debug)]
pub enum Message {
    TextInputChanged(String),
}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged(_) => todo!(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        // Context selector
        let context = Container::new(
            row![
                column![
                    text(CONTEXT_NAME),
                    row![
                        text("•").size(14).style(color!(0xDF5658)),
                        text(DISCONNECTED).size(14).style(color!(0xA7A9AD))
                    ]
                    .align_items(Alignment::Center)
                ]
                .width(Length::Fill),
                icon(Icon::ChevronExpand).size(14).style(color!(0xA7A9AD))
            ]
            .align_items(Alignment::Center)
            .width(Length::Fill),
        )
        // .style(ContextStyle)
        .padding(16);

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

        let omnibox = row![
            TextInput::new("Search", "", Message::TextInputChanged).style(TextInputStyle),
            Button::new(icon(Icon::Plus)).style(ButtonStyle.into())
        ]
        .padding([0, 0, 16, 0])
        .spacing(4);

        let content = column![
            context,
            column![omnibox, Scrollable::new(playbooks).style(ScrollableStyle)].padding(16)
        ];

        Container::new(content)
            .style(SidebarStyle)
            .height(Length::Fill)
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

impl Into<iced::theme::Container> for SidebarStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(SidebarStyle))
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

struct ContextStyle;

impl container::StyleSheet for ContextStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: color!(0x393D48).into(),
            ..Default::default()
        }
    }
}

impl Into<iced::theme::Container> for ContextStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(ContextStyle))
    }
}

struct TextInputStyle;

impl text_input::StyleSheet for TextInputStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: color!(0x292C33).into(),
            border_radius: 6.0,
            border_width: 1.0,
            border_color: color!(0x474B56),
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: color!(0x292C33).into(),
            border_radius: 6.0,
            border_width: 1.0,
            border_color: color!(0x474B56),
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        color!(0x474B56)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        color!(0x474B56)
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        color!(0x474B56)
    }
}

impl Into<iced::theme::TextInput> for TextInputStyle {
    fn into(self) -> iced::theme::TextInput {
        iced::theme::TextInput::Custom(Box::new(TextInputStyle))
    }
}

struct ButtonStyle;

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border_radius: 6.0,
            border_width: 1.0,
            border_color: color!(0x474B56),
            text_color: color!(0xffffff),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(color!(0x474B56).into()),
            border_radius: 6.0,
            border_width: 1.0,
            border_color: color!(0x474B56),
            text_color: color!(0xffffff),
            ..Default::default()
        }
    }
}

impl Into<iced::theme::Button> for ButtonStyle {
    fn into(self) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(ButtonStyle))
    }
}
