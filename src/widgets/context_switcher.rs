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

use crate::{
    styles::{self, Theme},
    widgets::Button,
};
use amp_common::config::Cluster;
use iced::{widget::Component, Alignment, Length};
use iced_aw::{
    menu::{Item, Menu, MenuBar},
    BootstrapIcon as Icon, BOOTSTRAP_FONT as ICON_FONT,
};
use tracing::debug;

use crate::utils::connection_status::ConnectionStatus;

use super::{Column, Container, Element, Row, Text};

#[derive(Default)]
pub struct ContextSwitcher<Message> {
    name: String,
    title: String,

    clusters: HashMap<String, Cluster>,
    status: ConnectionStatus,

    on_change: Option<Box<dyn Fn(String) -> Message>>,
}

#[derive(Clone)]
pub enum Event {
    ItemPressed(String),
}

impl<Message> ContextSwitcher<Message> {
    pub fn new(name: String, title: String, clusters: HashMap<String, Cluster>, status: ConnectionStatus) -> Self {
        Self {
            name,
            title,
            clusters,
            status,
            on_change: None,
        }
    }

    pub fn on_change(mut self, on_change: impl Fn(String) -> Message + 'static) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }
}

impl<Message> Component<Message, Theme> for ContextSwitcher<Message> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::ItemPressed(name) => {
                debug!("The context switcher pressed: {}", name);
                // If the name is the same as the current context, do nothing
                if name.eq(&self.name) {
                    return None;
                }
                self.on_change.as_ref().map(|f| f(name))
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        let style = match self.status {
            ConnectionStatus::Connecting => styles::Text::Secondary,
            ConnectionStatus::Connected => styles::Text::Success,
            ConnectionStatus::Disconnected => styles::Text::Danger,
        };
        let text = self.status.to_string();
        let state = Row::new()
            .push(
                Text::new("•")
                    .size(20)
                    .line_height(1.0)
                    .style(style)
                    .vertical_alignment(iced::alignment::Vertical::Top),
            )
            .push(Text::new(text).size(14).style(styles::Text::Secondary))
            .align_items(Alignment::Center);

        let title = Column::new()
            .push(Text::new(&self.title))
            .push(state)
            .width(Length::Fill)
            .into();

        let icon = Text::new(Icon::ChevronExpand.to_string())
            .style(styles::Text::Secondary)
            .font(ICON_FONT)
            .size(16.0)
            .into();

        let header = Row::with_children(vec![title, icon])
            .align_items(Alignment::Center)
            .width(Length::Fill);

        let items = self
            .clusters
            .iter()
            .map(|(name, cluster)| {
                Item::new(
                    Button::new(Text::new(&cluster.title))
                        .style(styles::Button::Menu)
                        .width(Length::Fill)
                        .on_press(Event::ItemPressed(name.clone())),
                )
            })
            .collect();

        let root_menu_items = Item::with_menu(header, Menu::new(items).max_width(190.0));
        let content = MenuBar::new(vec![root_menu_items]).width(Length::Fill);

        Container::new(content).width(Length::Fill).into()
    }
}

impl<'a, Message> From<ContextSwitcher<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(switcher: ContextSwitcher<Message>) -> Self {
        iced::widget::component(switcher)
    }
}
