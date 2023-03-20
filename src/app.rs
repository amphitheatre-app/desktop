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

use components::body::Body;
use components::sidebar::Sidebar;
use iced::theme::Palette;
use iced::{color, Element, Sandbox, Theme};
use iced_aw::{split, Split};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SidebarMessage,
    BodyMessage,
    SplitResized(u16),
}

pub struct App {
    sidebar: Sidebar,
    body: Body,
    divider_position: Option<u16>,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            sidebar: Default::default(),
            body: Default::default(),
            divider_position: Some(240),
        }
    }

    fn title(&self) -> String {
        String::from("Amphitheatre Desktop")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SidebarMessage => todo!(),
            Message::BodyMessage => todo!(),
            Message::SplitResized(position) => self.divider_position = Some(position),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Split::new(
            self.sidebar.view().map(|_| Message::SidebarMessage),
            self.body.view().map(|_| Message::BodyMessage),
            self.divider_position,
            split::Axis::Vertical,
            Message::SplitResized,
        )
        .min_size_first(240)
        .min_size_second(719)
        .spacing(1.0)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(Palette {
            background: color!(0x292C33),
            ..Theme::Dark.palette()
        })
    }
}
