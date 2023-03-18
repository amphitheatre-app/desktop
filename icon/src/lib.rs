pub use bootstrap::Icon;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::Text;
use iced::Font;

mod bootstrap;

use bootstrap::icon_to_char;

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../assets/fonts/bootstrap.ttf"),
};

pub fn icon<'a>(icon: Icon) -> Text<'a> {
    Text::new(icon_to_char(icon).to_string())
        .font(ICONS)
        .width(20)
        .vertical_alignment(Vertical::Center)
        .horizontal_alignment(Horizontal::Center)
        .size(20)
}
