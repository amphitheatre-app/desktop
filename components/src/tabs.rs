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

use iced::widget::Container;
use iced::{Element, Length};
use iced_aw::TabLabel;

pub trait Tab {
    type Message;

    fn title(&self) -> String;

    fn label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        Container::new(self.content())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
