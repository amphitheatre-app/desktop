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

use crate::widgets::Element;
use crate::widgets::Renderer;
use crate::widgets::Row;
use crate::widgets::{Button, Card, Column, Container, Scrollable, Text, TextInput};
use iced::widget::horizontal_space;
use iced::Alignment;
use iced::{alignment::Horizontal, widget::Component, Length};

pub struct Compose<Message> {
    form: Form,
    on_change: Box<dyn Fn(Form) -> Message>,
    on_cancel: Message,
    on_submit: Message,
}

#[derive(Clone)]
pub enum Event {
    TitleChanged(String),
    DescriptionChanged(String),
    RepositoryChanged(String),

    CancelButtonPressed,
    SubmitButtonPressed,
}

#[derive(Clone, Debug, Default)]
pub struct Form {
    title: String,
    description: String,
    repository: String,
}

impl<Message: Clone> Compose<Message> {
    pub fn new(
        form: Form,
        on_change: impl Fn(Form) -> Message + 'static,
        on_cancel: Message,
        on_submit: Message,
    ) -> Self {
        Self {
            form,
            on_change: Box::new(on_change),
            on_cancel,
            on_submit,
        }
    }
}

impl<Message: Clone> Component<Message, Renderer> for Compose<Message> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::TitleChanged(title) => {
                self.form.title = title;
                Some((self.on_change)(self.form.clone()))
            }
            Event::DescriptionChanged(description) => {
                self.form.description = description;
                Some((self.on_change)(self.form.clone()))
            }
            Event::RepositoryChanged(repository) => {
                self.form.repository = repository;
                Some((self.on_change)(self.form.clone()))
            }
            Event::CancelButtonPressed => Some(self.on_cancel.clone()),
            Event::SubmitButtonPressed => Some(self.on_submit.clone()),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        let element = Card::new(
            Text::new("Compose a new playbook")
                .size(20)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Left),
            self.form(),
        )
        .foot(self.actions())
        .padding(16.0);

        let content = Scrollable::new(element);
        Container::new(Column::new().push(content).max_width(480))
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}

impl<Message> Compose<Message> {
    fn form(&self) -> Element<Event> {
        let title = Column::with_children(vec![
            Text::new("Add a title").into(),
            TextInput::new("Title", &self.form.title)
                .on_input(Event::TitleChanged)
                .into(),
        ])
        .into();

        let description = Column::with_children(vec![
            Text::new("Add a description").into(),
            TextInput::new("Add your description here...", &self.form.description)
                .on_input(Event::DescriptionChanged)
                .into(),
        ])
        .into();

        let repository = Column::with_children(vec![
            Text::new("Repository").into(),
            TextInput::new("An SSH URL, like git@github.com:user/repo.git", &self.form.repository)
                .on_input(Event::RepositoryChanged)
                .into(),
        ])
        .into();

        Column::with_children(vec![title, description, repository])
            .spacing(16)
            .into()
    }

    fn actions(&self) -> Element<Event> {
        let cancel_button = Button::new(Text::new("Cancel")).on_press(Event::CancelButtonPressed);
        let submit_button = Button::new(Text::new("Submit")).on_press(Event::SubmitButtonPressed);

        Container::new(
            Row::new()
                .push(cancel_button)
                .push(horizontal_space(Length::Fill))
                .push(submit_button)
                .width(Length::Fill)
                .align_items(Alignment::Center),
        )
        .padding(16)
        .into()
    }
}

impl<'a, Message> From<Compose<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(component: Compose<Message>) -> Self {
        iced::widget::component(component)
    }
}
