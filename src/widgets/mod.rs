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

pub mod tabs;

use crate::styles::Theme;

pub type Renderer = iced::Renderer<Theme>;

pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;

pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;

pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;

pub type Text<'a> = iced::widget::Text<'a, Renderer>;

pub type TextInput<'a, Message, Renderer = self::Renderer> = iced::widget::TextInput<'a, Message, Renderer>;

pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;

pub type Modal<'a, Message, Renderer = self::Renderer> = iced_aw::Modal<'a, Message, Renderer>;

pub type Card<'a, Message, Renderer = self::Renderer> = iced_aw::Card<'a, Message, Renderer>;

pub type Tabs<'a, Message, TabId, Renderer = self::Renderer> = iced_aw::Tabs<'a, Message, TabId, Renderer>;

pub type Split<'a, Message, Renderer = self::Renderer> = iced_aw::Split<'a, Message, Renderer>;

pub type Scrollable<'a, Message, Renderer = self::Renderer> = iced::widget::Scrollable<'a, Message, Renderer>;

pub type Button<'a, Message, Renderer = self::Renderer> = iced::widget::Button<'a, Message, Renderer>;
