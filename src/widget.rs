use crate::theme::Theme;

pub type Renderer = iced::Renderer<Theme>;

pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;

pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;

pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;

pub type Text<'a> = iced::widget::Text<'a, Renderer>;

pub type TextInput<'a, Message, Renderer = self::Renderer> = iced::widget::TextInput<'a, Message, Renderer>;

pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;

pub type Modal<'a, Content, Message, Backend, Theme> = iced_aw::Modal<'a, Content, Message, Backend, Theme>;

pub type Card<'a, Message, Backend, Theme> = iced_aw::Card<'a, Message, Backend, Theme>;

pub type Tabs<'a, Message, Backend, Theme> = iced_aw::Tabs<'a, Message, Backend, Theme>;

pub type Split<'a, Message, Backend, Theme> = iced_aw::Split<'a, Message, Backend, Theme>;

pub type Scrollable<'a, Message, Renderer = self::Renderer> = iced::widget::Scrollable<'a, Message, Renderer>;

pub type Button<'a, Message, Renderer = self::Renderer> = iced::widget::Button<'a, Message, Renderer>;
