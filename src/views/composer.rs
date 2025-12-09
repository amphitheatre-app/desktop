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

use std::path::Path;

use iced::widget::space;
use iced::{Alignment, Length};
use iced_aw::Card;
use native_dialog::DialogBuilder;

use crate::styles::{self, constants::*};
use crate::widgets::{Button, Checkbox, Column, Container, Element, Row, Scrollable, Text, TextInput};

#[derive(Default)]
pub struct Composer {
    form: Form,
}

#[derive(Debug, Clone)]
pub enum Message {
    TitleChanged(String),
    DescriptionChanged(String),

    RepositoryChanged(String),
    SelectFileButtonPressed,
    LiveUpdateChecked(bool),

    CancelButtonPressed,
    SubmitButtonPressed,
}

pub enum Action {
    None,
    Submit(Form),
    Cancel,
}

#[derive(Clone, Debug, Default)]
pub struct Form {
    pub title: String,
    pub description: String,
    pub preface: String,
    pub live: bool,
}

impl Composer {
    pub fn reset(&mut self) {
        self.form = Form::default();
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::TitleChanged(title) => {
                self.form.title = title;
                Action::None
            }
            Message::DescriptionChanged(description) => {
                self.form.description = description;
                Action::None
            }
            Message::RepositoryChanged(repository) => {
                self.form.preface = repository;
                Action::None
            }
            Message::SelectFileButtonPressed => {
                if let Ok(Some(path)) = DialogBuilder::file().open_single_dir().show() {
                    self.form.preface = path.to_str().unwrap_or_default().to_string();
                }
                Action::None
            }
            Message::LiveUpdateChecked(live) => {
                self.form.live = live;
                Action::None
            }
            Message::CancelButtonPressed => Action::Cancel,
            Message::SubmitButtonPressed => Action::Submit(self.form.clone()),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = Text::new("Compose a new playbook").size(FONT_SIZE_LARGE);
        let element = Card::new(title, self.form())
            .close_size(ICON_FONT_SIZE_TOOLBAR)
            .on_close(Message::CancelButtonPressed)
            .foot(self.actions())
            .padding(SPACING_LARGE.into());

        let content = Scrollable::new(element);
        Container::new(Column::new().push(content).max_width(480))
            .center(Length::Fill)
            .into()
    }

    fn form(&self) -> Element<'_, Message> {
        let help = Text::new(
            "Please give your playbook a clear title and description to convey its purpose \
            and content effectively. Additionally, provide a GIT URL or local file path as the repository.",
        )
        .style(styles::text::secondary)
        .into();

        let title = Column::with_children(vec![
            Text::new("Name your playbook").into(),
            TextInput::new("Untitled", &self.form.title)
                .on_input(Message::TitleChanged)
                .into(),
        ])
        .into();

        let description = Column::with_children(vec![
            Text::new("Add a description").into(),
            TextInput::new("Add your description here...", &self.form.description)
                .on_input(Message::DescriptionChanged)
                .into(),
        ])
        .into();

        let repo_placeholder = "The repository to be cloned, or the local path to the project";
        let repository = Column::with_children(vec![
            Text::new("Repository").into(),
            Row::new()
                .push(TextInput::new(repo_placeholder, &self.form.preface).on_input(Message::RepositoryChanged))
                .push(Button::new(Text::new("Browse")).on_press(Message::SelectFileButtonPressed))
                .spacing(SPACING_SMALL)
                .into(),
        ])
        .into();

        let mut fields = vec![help, title, description, repository];
        if Path::new(&self.form.preface).exists() {
            fields.push(
                Checkbox::new(self.form.live)
                    .label("Running in development mode")
                    .on_toggle(Message::LiveUpdateChecked)
                    .into(),
            );
        }

        Column::with_children(fields).spacing(SPACING_LARGE).into()
    }

    fn actions(&self) -> Element<'_, Message> {
        let cancel_button = Button::new(Text::new("Cancel").style(styles::text::secondary))
            .style(styles::button::text)
            .on_press(Message::CancelButtonPressed);
        let submit_button = Button::new(Text::new("Start compose"))
            .style(styles::button::primary)
            .width(Length::FillPortion(3))
            .on_press(Message::SubmitButtonPressed);

        Container::new(
            Row::new()
                .push(cancel_button)
                .push(space::horizontal())
                .push(submit_button)
                .width(Length::Fill)
                .align_y(Alignment::Center),
        )
        .into()
    }
}
