use crate::theme::Theme;

pub type Renderer = iced::Renderer<Theme>;

pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;

pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;

pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;

pub type Text<'a> = iced::widget::Text<'a, Renderer>;

pub type TextInput<'a, Message, Renderer = self::Renderer> = iced::widget::TextInput<'a, Message, Renderer>;

pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;

pub type Modal<'a, Message, Renderer = self::Renderer> = iced_aw::Modal<'a, Message, Renderer>;

pub type Card<'a, Message, Renderer = self::Renderer> = iced_aw::Card<'a, Message, Renderer>;

pub type Tabs<'a, Message, TabId, Renderer = self::Renderer> = iced_aw::Tabs<'a, Message, TabId, Renderer>;

pub type Split<'a, Message, Renderer = self::Renderer> = iced_aw::Split<'a, Message, Renderer>;

pub type Scrollable<'a, Message, Renderer = self::Renderer> = iced::widget::Scrollable<'a, Message, Renderer>;

pub type Button<'a, Message, Renderer = self::Renderer> = iced::widget::Button<'a, Message, Renderer>;
