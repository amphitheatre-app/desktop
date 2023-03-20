use iced::widget::{button, container, rule, scrollable, text, text_input};
use iced::{application, color, Color};
use iced_aw::style::tab_bar;
use iced_aw::{split, tabs};

const SURFACE: Color = Color::from_rgb(0x29 as f32 / 255.0, 0x2C as f32 / 255.0, 0x33 as f32 / 255.0);
const ACCENT: Color = Color::from_rgb(0x30 as f32 / 255.0, 0x34 as f32 / 255.0, 0x3D as f32 / 255.0);

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: SURFACE,
            text_color: color!(0xC9CCD3),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Text {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        let color = match style {
            Text::Primary => color!(0xC9CCD3),
            Text::Secondary => color!(0x757983),
            Text::Success => color!(0x49914C),
            Text::Danger => color!(0xDF5658),
        };

        text::Appearance { color: Some(color) }
    }
}

impl split::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: Self::Style) -> split::Appearance {
        split::Appearance {
            first_background: Some(ACCENT.into()),
            divider_background: ACCENT.into(),
            divider_border_color: ACCENT.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            ..split::Appearance::default()
        }
    }

    fn hovered(&self, style: Self::Style) -> split::Appearance {
        self.active(style)
    }

    fn dragged(&self, style: Self::Style) -> split::Appearance {
        self.active(style)
    }
}

impl tabs::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: Self::Style, is_active: bool) -> tab_bar::Appearance {
        let mut appearance = tab_bar::Appearance::default();

        let text_color = if is_active {
            color!(0xC9CCD3)
        } else {
            color!(0x848993)
        };

        appearance.tab_label_background = SURFACE.into();
        appearance.tab_label_border_width = 0.0;
        appearance.tab_label_border_color = Color::TRANSPARENT;
        appearance.icon_color = text_color;
        appearance.text_color = text_color;

        appearance
    }

    fn hovered(&self, style: Self::Style, is_active: bool) -> tab_bar::Appearance {
        tab_bar::Appearance {
            icon_color: color!(0xC9CCD3),
            text_color: color!(0xC9CCD3),
            ..self.active(style, is_active)
        }
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Default::default(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
            scroller: scrollable::Scroller {
                color: Default::default(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Default::default(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
            scroller: scrollable::Scroller {
                color: Default::default(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
            },
        }
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: color!(0x474B56),
            width: 1,
            radius: 0.0,
            fill_mode: rule::FillMode::Full,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance::default()
    }
}

impl text_input::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: color!(0x292C33).into(),
            border_radius: 6.0,
            border_width: 1.0,
            border_color: color!(0x474B56),
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: color!(0x292C33).into(),
            border_radius: 6.0,
            border_width: 1.0,
            border_color: color!(0x474B56),
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        color!(0x474B56)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        color!(0xC9CCD3)
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        color!(0x474B56)
    }
}
impl button::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border_radius: 6.0,
            border_width: 1.0,
            border_color: color!(0x474B56),
            text_color: color!(0xffffff),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(color!(0x474B56).into()),
            border_radius: 6.0,
            border_width: 1.0,
            border_color: color!(0x474B56),
            text_color: color!(0xffffff),
            ..button::Appearance::default()
        }
    }
}
