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

use crate::{
    styles::{self, constants::ICON_FONT_SIZE_TOOLBAR, Theme},
    widgets::{Button, Container, Element, Text},
};
use amp_common::resource::CharacterSpec;
use iced::{widget::Component, Length};
use iced_aw::menu::{Item, Menu, MenuBar};
use iced_aw::{core::icons::bootstrap::Bootstrap as Icon, BOOTSTRAP_FONT as ICON_FONT};
use tracing::debug;

#[derive(Default)]
pub struct CharacterSwitcher<Message> {
    current: CharacterSpec,
    characters: Vec<CharacterSpec>,
    on_change: Option<Box<dyn Fn(CharacterSpec) -> Message>>,
}

#[derive(Clone)]
pub enum Event {
    ItemPressed(CharacterSpec),
}

impl<Message> CharacterSwitcher<Message> {
    pub fn new(current: CharacterSpec, characters: Vec<CharacterSpec>) -> Self {
        Self {
            current,
            characters,
            on_change: None,
        }
    }

    pub fn on_change(mut self, on_change: impl Fn(CharacterSpec) -> Message + 'static) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }
}

impl<Message> Component<Message, Theme> for CharacterSwitcher<Message> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::ItemPressed(character) => {
                debug!("The character switcher pressed: {}", character.meta.name);
                // If the name is the same as the current context, do nothing
                if character.eq(&self.current) {
                    return None;
                }
                self.on_change.as_ref().map(|f| f(character))
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        let icon = Text::new(Icon::List.to_string())
            .font(ICON_FONT)
            .size(ICON_FONT_SIZE_TOOLBAR);

        let items = self
            .characters
            .iter()
            .map(|character| {
                Item::new(
                    Button::new(Text::new(&character.meta.name))
                        .style(styles::Button::Menu)
                        .width(Length::Fill)
                        .on_press(Event::ItemPressed(character.clone())),
                )
            })
            .collect();

        let root_menu_items = Item::with_menu(icon, Menu::new(items).max_width(190.0));
        let content = MenuBar::new(vec![root_menu_items]).width(Length::Fill);

        Container::new(content).into()
    }
}

impl<'a, Message> From<CharacterSwitcher<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(switcher: CharacterSwitcher<Message>) -> Self {
        iced::widget::component(switcher)
    }
}
