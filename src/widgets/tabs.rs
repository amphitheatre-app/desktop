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

use iced::Length;
use iced_aw::TabLabel;

use crate::widgets::{Column, Container, Element, Rule};

pub trait Tab {
    type Message;

    fn title(&self) -> String;
    fn label(&self) -> TabLabel;

    fn content(&self) -> Element<'_, Self::Message> {
        Container::new(
            Column::new().push(Rule::horizontal(1)).push(
                Container::new(self.view())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(16),
            ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn view(&self) -> Element<'_, Self::Message>;
}
