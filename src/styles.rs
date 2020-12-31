use iced::{button, Background, Color};

const BUTTON_BORDER: f32 = 5.;

const PRINCESS_COLOR: Color = Color::from_rgba(1.00, 0.90, 0.98, 0.5);
const FAIRY_COLOR: Color = Color::from_rgba(0.70, 0.75, 1.00, 0.5);
const ANGEL_COLOR: Color = Color::from_rgba(1.00, 1.00, 0.30, 0.5);
const EXTRA_COLOR: Color = Color::from_rgba(0.75, 1.00, 0.70, 0.5);

const PRINCESS_BORDER: Color = Color::from_rgba(1.00, 0.70, 0.95, 0.75);
const FAIRY_BORDER: Color = Color::from_rgba(0.60, 0.67, 1.00, 0.75);
const ANGEL_BORDER: Color = Color::from_rgba(1.00, 1.00, 0.20, 0.75);
const EXTRA_BORDER: Color = Color::from_rgba(0.67, 1.00, 0.60, 0.75);

const HOVER_TEXT: Color = Color::from_rgba(0., 0., 0., 0.75);

pub struct CardButtonStyle {
    pub idol_type: i8
}

impl CardButtonStyle {
    fn match_color(&self) -> Color {
        let color = match self.idol_type {
            1 => PRINCESS_COLOR,
            2 => FAIRY_COLOR,
            3 => ANGEL_COLOR,
            5 => EXTRA_COLOR,
            _ => Color::WHITE
        };

        color
    }

    fn match_border(&self) -> Color {
        let color = match self.idol_type {
            1 => PRINCESS_BORDER,
            2 => FAIRY_BORDER,
            3 => ANGEL_BORDER,
            5 => EXTRA_BORDER,
            _ => Color::WHITE
        };

        color
    }
}

impl button::StyleSheet for CardButtonStyle {
    fn active(&self) -> button::Style {
        let color = self.match_color();

        button::Style {
            background: Some(Background::Color(color)),
            border_radius: BUTTON_BORDER,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        let color = self.match_color();
        let border_color = self.match_border();

        button::Style {
            background: Some(Background::Color(color)),
            border_width: 1.,
            border_radius: BUTTON_BORDER,
            border_color: border_color,
            text_color: HOVER_TEXT,
            ..button::Style::default()
        }
    }
}