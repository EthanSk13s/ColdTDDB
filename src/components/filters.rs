use iced::{Checkbox, Row, Element, Text};
use crate::app::Message;

pub struct TypeFilter {
    pub princess_toggle: bool,
    pub fairy_toggle: bool,
    pub angel_toggle: bool,
    pub extra_toggle: bool
}

impl TypeFilter {
    pub fn new() -> Self {
        TypeFilter {
            princess_toggle: true,
            fairy_toggle: true,
            angel_toggle: true,
            extra_toggle: true
        }
    }

    pub fn set_state(&mut self, value: i32, state: bool) {
        match value {
            1 => self.princess_toggle = state,
            2 => self.fairy_toggle = state,
            3 => self.angel_toggle = state,
            5 => self.extra_toggle = state,
            _ => (),
        };
    }
}

#[derive(Debug, Clone)]
pub struct RarityFilter {
    pub n_toggle: bool,
    pub r_toggle: bool,
    pub sr_toggle: bool,
    pub ssr_toggle: bool
}

impl RarityFilter {
    pub fn new() -> Self {
        RarityFilter {
            n_toggle: true,
            r_toggle: true,
            sr_toggle: true,
            ssr_toggle: true
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let rarity_row = Row::new().push(Text::new("Rarity: "));

        let n_radio = Checkbox::new(
            self.n_toggle,
            "N",
            move|toggle| {Message::ToggleRarity(toggle, 1)}
        );

        let r_radio = Checkbox::new(
            self.r_toggle,
            "R",
            move|toggle| {Message::ToggleRarity(toggle, 2)}
        );

        let sr_radio = Checkbox::new(
            self.sr_toggle,
            "SR",
            move|toggle| {Message::ToggleRarity(toggle, 3)}
        );

        let ssr_radio = Checkbox::new(
            self.ssr_toggle,
            "SSR",
            move|toggle| {Message::ToggleRarity(toggle, 4)}
        );

        rarity_row
            .push(n_radio)
            .push(r_radio)
            .push(sr_radio)
            .push(ssr_radio)
            .into()

    }

    pub fn set_state(&mut self, value: i32, state: bool) {
        match value {
            1 => self.n_toggle = state,
            2 => self.r_toggle = state,
            3 => self.sr_toggle = state,
            4 => self.ssr_toggle = state,
            _ => (),
        };
    }
}
