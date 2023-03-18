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

use iced::widget::{column, container, row, text, Container, Scrollable, Text};
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
        .padding([20, 0])
        .align_items(Alignment::Center)
        .width(Length::Fill);

        let content = column![context, text("Sidebar")];

        Container::new(Scrollable::new(content))
            .style(theme::Container::Custom(Box::new(SidebarStyle)))
            .width(240.0)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
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
