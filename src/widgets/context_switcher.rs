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

use amp_common::config::ContextConfiguration;
use iced::{widget::Component, Alignment, Length};
use iced_aw::{Icon, ICON_FONT};
use tracing::debug;

use crate::{styles, utils::connection_status::ConnectionStatus};

use super::{Button, Column, Container, Element, Renderer, Row, Text};

#[derive(Default)]
pub struct ContextSwitcher<Message> {
    context: ContextConfiguration,
    status: ConnectionStatus,

    on_change: Option<Box<dyn Fn(String) -> Message>>,
}

#[derive(Clone)]
pub enum Event {
    ButtonPressed,
}

impl<Message> ContextSwitcher<Message> {
    pub fn new(context: ContextConfiguration, status: ConnectionStatus) -> Self {
        Self {
            context,
            status,
            on_change: None,
        }
    }

    pub fn on_change(mut self, on_change: impl Fn(String) -> Message + 'static) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }
}

impl<Message> Component<Message, Renderer> for ContextSwitcher<Message> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::ButtonPressed => {
                debug!("The context switcher pressed");
                None
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
            .push(Text::new("•").size(14).style(style))
            .push(Text::new(text).size(14).style(styles::Text::Secondary))
            .align_items(Alignment::Center);

        let title = self.context.current().map(|c| c.title.as_str()).unwrap_or("UNKNOWN");
        let heading = Column::new().push(Text::new(title)).push(state).width(Length::Fill);

        Container::new(
            Button::new(
                Row::new()
                    .push(heading)
                    .push(Text::new(Icon::ChevronExpand.to_string()).font(ICON_FONT).size(16.0))
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .style(styles::Button::Element)
            .on_press(Event::ButtonPressed),
        )
        .into()
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
