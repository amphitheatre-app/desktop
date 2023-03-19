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

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, horizontal_space, row, rule, text, vertical_rule, Container};
use iced::{color, Alignment, Color, Element, Length, Theme};
use icon::{icon, Icon};

#[derive(Debug, Default)]
pub struct Body {}

#[derive(Clone)]
pub enum Message {
    ButtonPressed,
}

impl Body {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<Message> {
        let title = row![
            icon(Icon::List),
            column![
                text("Clean code linters"),
                text("Running").size(14).style(color!(0x49914C)),
            ],
        ]
        .align_items(Alignment::Center)
        .spacing(4);

        let actions = row![
            button(icon(Icon::Play))
                .style(ButtonStyle.into())
                .on_press(Message::ButtonPressed),
            button(icon(Icon::Stop))
                .style(ButtonStyle.into())
                .on_press(Message::ButtonPressed),
            button(icon(Icon::ArrowRepeat))
                .style(ButtonStyle.into())
                .on_press(Message::ButtonPressed),
            button(icon(Icon::X))
                .style(ButtonStyle.into())
                .on_press(Message::ButtonPressed)
        ]
        .align_items(Alignment::Center)
        .spacing(4);

        // toolbar

        let toolbar = Container::new(
            row![title, horizontal_space(Length::Fill), actions]
                .width(Length::Fill)
                .align_items(Alignment::Center),
        )
        .style(ToolbarStyle)
        .padding(16);

        // tabs
        let tabs = Container::new(
            row![
                Container::new(
                    text("Logs")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .style(ActiveTabStyle),
                vertical_rule(0.4).style(RuleStyle),
                text("Resources")
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .style(color!(0xA7A9AD)),
                vertical_rule(0.4).style(RuleStyle),
                text("Envrionment")
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .style(color!(0xA7A9AD)),
                vertical_rule(0.4).style(RuleStyle),
                text("Stats")
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .style(color!(0xA7A9AD)),
            ]
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .height(32)
        .style(TabsStyle);

        let content = column![toolbar, tabs, horizontal_space(Length::Shrink)];
        Container::new(content).into()
    }
}

struct ToolbarStyle;

impl container::StyleSheet for ToolbarStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: color!(0x393D48).into(),
            ..Default::default()
        }
    }
}

impl Into<iced::theme::Container> for ToolbarStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(ToolbarStyle))
    }
}

struct TabsStyle;

impl container::StyleSheet for TabsStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: color!(0x30343D).into(),
            ..Default::default()
        }
    }
}

impl Into<iced::theme::Container> for TabsStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(TabsStyle))
    }
}

struct ActiveTabStyle;

impl container::StyleSheet for ActiveTabStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: color!(0x292C33).into(),
            ..Default::default()
        }
    }
}

impl Into<iced::theme::Container> for ActiveTabStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(ActiveTabStyle))
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
