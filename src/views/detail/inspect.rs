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

use std::collections::HashMap;
use std::sync::Arc;

use amp_client::playbooks::Playbook;
use iced::widget::Rule;
use iced::{Command, Length, Subscription};
use iced_aw::TabLabel;

use crate::context::Context;
use crate::widgets::tabs::Tab;
use crate::widgets::{Column, Element, Row, Scrollable, Text};

#[derive(Clone, Debug)]
pub enum Message {}

pub struct Information {
    data: HashMap<String, HashMap<String, String>>,
}

impl Information {
    pub fn new(_ctx: Arc<Context>, _playbook: Playbook) -> Self {
        Self {
            data: HashMap::from([
                (
                    "environments".into(),
                    HashMap::from([
                        ("K3S_TOKEN".into(), "RdqNLMXRiRsHJhmxKurR".into()),
                        ("K3S_KUBECONFIG_OUTPUT".into(), "/output/kubeconfig.yaml".into()),
                        (
                            "PATH".into(),
                            "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/bin/aux".into(),
                        ),
                        ("CRI_CONFIG_FILE".into(), "/var/lib/rancher/k3s/agent/etc/crictl.yaml".into()),
                    ]),
                ),
                (
                    "mounts".into(),
                    HashMap::from([
                        (
                            "/VAR/LIB/CNI".into(),
                            "/var/lib/docker/volumes/00f49631b07ccd74de44d3047d5f889395ac871e05b622890b6dd788d34a59f4/_data".into(),
                        ),
                        (
                            "/VAR/LIB/KUBELET".into(),
                            "/var/lib/docker/volumes/bc1b16d39a0e204841695de857122412cfdefd0f672af185b1fa43e635397848/_data".into(),
                        ),
                        (
                            "/VAR/LIB/RANCHER/K3S".into(),
                            "/var/lib/docker/volumes/a78bcb9f7654701e0cfaef4447ef61ced4864e5b93dee7102ec639afb5cf2e1d/_data".into(),
                        ),
                        (
                            "/VAR/LOG".into(),
                            "/var/lib/docker/volumes/f64c2f2cf81cfde89879f2a17924b31bd2f2e6a6a738f7df949bf6bd57102d25/_data".into(),
                        ),
                    ]),
                ),
                ("port".into(), HashMap::from([("6443/tcp".into(), "0.0.0.0:42397".into())])),
            ])
        }
    }

    pub fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn view(&self) -> Element<Message> {
        let mut children = vec![];

        for (group, fields) in &self.data {
            children.push(Text::new(group.to_ascii_uppercase()).size(24).into());
            for (key, value) in fields {
                children.push(
                    Column::new()
                        .push(
                            Row::new()
                                .push(Text::new(key).size(16).width(Length::FillPortion(4)))
                                .push(Text::new(value).size(14).width(Length::FillPortion(6)))
                                .width(Length::Fill),
                        )
                        .push(Rule::horizontal(1))
                        .width(Length::Fill)
                        .spacing(16)
                        .into(),
                );
            }
        }

        let content = Column::with_children(children)
            .padding(16)
            .spacing(16)
            .width(Length::Fill);
        Scrollable::new(content).into()
    }
}

impl Tab for Information {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Inspect")
    }

    fn label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    #[inline]
    fn view(&self) -> Element<Self::Message> {
        self.view()
    }
}
