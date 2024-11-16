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

use amp_common::config::Cluster;
use iced::{Alignment, Length};
use iced_fonts::{Bootstrap as Icon, BOOTSTRAP_FONT as ICON_FONT};
use tracing::debug;

use crate::styles;
use crate::utils::connection_status::ConnectionStatus;
use crate::widgets::menu::{Item, Menu, MenuBar};
use crate::widgets::{Button, Column, Container, Element, Row, Text};

#[derive(Default)]
pub struct ContextSwitcher {
    name: String,
    title: String,

    clusters: HashMap<String, Cluster>,
    status: ConnectionStatus,
}

#[derive(Debug, Clone)]
pub enum Message {
    ItemPressed(String),
}

pub enum Action {
    None,
    Switch(String),
}

impl ContextSwitcher {
    pub fn new(name: String, title: String, clusters: HashMap<String, Cluster>, status: ConnectionStatus) -> Self {
        Self {
            name,
            title,
            clusters,
            status,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ItemPressed(name) => {
                debug!("The context switcher pressed: {}", name);
                // If the name is the same as the current context, do nothing
                if name.eq(&self.name) {
                    return Action::None;
                }
                Action::Switch(name)
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let style = match self.status {
            ConnectionStatus::Connecting => styles::text::primary,
            ConnectionStatus::Connected => styles::text::success,
            ConnectionStatus::Disconnected => styles::text::danger,
        };
        let text = self.status.to_string();
        let state = Row::new()
            .push(Text::new("•").size(20).line_height(1.0).style(style))
            .push(Text::new(text).size(14).style(styles::text::secondary))
            .align_y(Alignment::Center);

        let title = Column::new()
            .push(Text::new(&self.title))
            .push(state)
            .width(Length::Fill)
            .into();

        let icon = Text::new(Icon::ChevronExpand.to_string())
            .style(styles::text::secondary)
            .font(ICON_FONT)
            .size(16.0)
            .into();

        let header = Row::with_children(vec![title, icon])
            .align_y(Alignment::Center)
            .width(Length::Fill);

        let items = self
            .clusters
            .iter()
            .map(|(name, cluster)| {
                Item::new(
                    Button::new(Text::new(&cluster.title))
                        .style(styles::button::text)
                        .width(Length::Fill)
                        .on_press(Message::ItemPressed(name.clone())),
                )
            })
            .collect();

        let root_menu_items = Item::with_menu(header, Menu::new(items).max_width(190.0));
        let content = MenuBar::new(vec![root_menu_items]).width(Length::Fill);

        Container::new(content).width(Length::Fill).into()
    }
}
