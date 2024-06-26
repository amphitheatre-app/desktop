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

use amp_common::resource::CharacterSpec;
use iced::widget::Component;
use iced::Alignment;
use iced::Length;
use iced_aw::{core::icons::bootstrap::Bootstrap as Icon, BOOTSTRAP_FONT as ICON_FONT};

use crate::styles;
use crate::styles::constants::ICON_FONT_SIZE_SIDEBAR;
use crate::styles::Theme;
use crate::widgets::Button;
use crate::widgets::Element;
use crate::widgets::Row;
use crate::widgets::Text;

pub struct CharacterItem<Message> {
    character: CharacterSpec,
    active: bool,
    on_press: Option<Box<dyn Fn(CharacterSpec) -> Message>>,
}

impl<Message> CharacterItem<Message> {
    pub fn new(character: CharacterSpec) -> Self {
        Self {
            character,
            active: false,
            on_press: None,
        }
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Button`] will be disabled.
    pub fn on_press(mut self, on_press: impl Fn(CharacterSpec) -> Message + 'static) -> Self {
        self.on_press = Some(Box::new(on_press));
        self
    }

    /// Sets the active state of the [`Button`].
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

#[derive(Clone)]
pub enum Event {
    ButtonPressed,
}

impl<Message> Component<Message, Theme> for CharacterItem<Message> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::ButtonPressed => self.on_press.as_ref().map(|f| f(self.character.clone())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        let icon = Text::new(Icon::Box.to_string())
            .font(ICON_FONT)
            .size(ICON_FONT_SIZE_SIDEBAR);

        let content = Row::new()
            .push(icon)
            .push(Text::new(&self.character.meta.name))
            .align_items(Alignment::Center)
            .spacing(8);

        Button::new(content)
            .style(if self.active {
                styles::Button::Primary
            } else {
                styles::Button::Element
            })
            .width(Length::Fill)
            .on_press(Event::ButtonPressed)
            .into()
    }
}

impl<'a, Message> From<CharacterItem<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(item: CharacterItem<Message>) -> Self {
        iced::widget::component(item)
    }
}
