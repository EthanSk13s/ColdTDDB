use iced::{button, Background, Color};

pub struct CardButtonStyle;

impl button::StyleSheet for CardButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            border_width: 1.,
            border_radius: 5.,
            border_color: Color::from_rgb(0.70,0.80,1.00),
            text_color: Color::from_rgba(0., 0., 0., 0.75),
            ..button::Style::default()
        }
    }
}