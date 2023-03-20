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

use iced_aw::TabLabel;

use crate::components::tabs::Tab;
use crate::widget::{Container, Element, Text};

#[derive(Clone, Debug)]
pub enum Message {}

#[derive(Default)]
pub struct Resources;

impl Resources {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: Message) {}
}

impl Tab for Resources {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Resources")
    }

    fn label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        Container::new(Text::new(self.title())).into()
    }
}
