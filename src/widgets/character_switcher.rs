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

use std::sync::Arc;

use iced::Length;
use iced_aw::menu::{Item, Menu, MenuBar};
use iced_fonts::{Bootstrap as Icon, BOOTSTRAP_FONT as ICON_FONT};
use tracing::debug;

use amp_common::resource::{CharacterSpec, PlaybookSpec};

use crate::styles;
use crate::styles::constants::ICON_FONT_SIZE_TOOLBAR;
use crate::widgets::{Button, Container, Element, Text};

#[derive(Default)]
pub struct CharacterSwitcher {
    playbook: Arc<PlaybookSpec>,
    current: Arc<CharacterSpec>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ItemPressed(Arc<CharacterSpec>),
}

pub enum Action {
    None,
    Switch(Arc<CharacterSpec>),
}

impl CharacterSwitcher {
    pub fn new(playbook: Arc<PlaybookSpec>, current: Arc<CharacterSpec>) -> Self {
        Self { playbook, current }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ItemPressed(character) => {
                debug!("The character switcher pressed: {}", character.meta.name);
                // If the name is the same as the current context, do nothing
                if character.eq(&self.current) {
                    return Action::None;
                }
                self.current = character;
                Action::Switch(self.current.clone())
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let icon = Text::new(Icon::List.to_string())
            .font(ICON_FONT)
            .size(ICON_FONT_SIZE_TOOLBAR);

        let items = if let Some(characters) = &self.playbook.characters {
            characters
                .iter()
                .map(|character| {
                    Item::new(
                        Button::new(Text::new(&character.meta.name))
                            .style(styles::button::text)
                            .width(Length::Fill)
                            .on_press(Message::ItemPressed(character.clone().into())),
                    )
                })
                .collect()
        } else {
            vec![]
        };

        let root_menu_items = Item::with_menu(icon, Menu::new(items).max_width(190.0));
        let content = MenuBar::new(vec![root_menu_items]).width(Length::Fill);

        Container::new(content).into()
    }
}
