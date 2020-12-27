use iced::{Checkbox, Row, Element, Text};
use crate::app::Message;

#[derive(Debug, Clone)]
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

    pub fn view(&mut self) -> Element<Message> {
        let type_row = Row::new().push(Text::new("Idol type:"));

        let princess_toggle = Checkbox::new(
            self.princess_toggle,
            "Princess",
            move|toggle| {Message::ToggleType(toggle, 1)}
        )
        .spacing(5);

        let fairy_toggle = Checkbox::new(
            self.fairy_toggle,
            "Fairy",
            move|toggle| {Message::ToggleType(toggle, 2)}
        )
        .spacing(5);

        let angel_toggle = Checkbox::new(
            self.angel_toggle,
            "Angel",
            move|toggle| {Message::ToggleType(toggle, 3)}
        )
        .spacing(5);

        let extra_toggle = Checkbox::new(
            self.extra_toggle,
            "Extra",
            move|toggle| {Message::ToggleType(toggle, 5)}
        )
        .spacing(5);

        type_row
            .spacing(10)
            .push(princess_toggle)
            .push(fairy_toggle)
            .push(angel_toggle)
            .push(extra_toggle)
            .into()
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
        let rarity_row = Row::new().push(Text::new("Rarity:"));

        let n_radio = Checkbox::new(
            self.n_toggle,
            "N",
            move|toggle| {Message::ToggleRarity(toggle, 1)}
        )
        .spacing(5);

        let r_radio = Checkbox::new(
            self.r_toggle,
            "R",
            move|toggle| {Message::ToggleRarity(toggle, 2)}
        )
        .spacing(5);

        let sr_radio = Checkbox::new(
            self.sr_toggle,
            "SR",
            move|toggle| {Message::ToggleRarity(toggle, 3)}
        )
        .spacing(5);

        let ssr_radio = Checkbox::new(
            self.ssr_toggle,
            "SSR",
            move|toggle| {Message::ToggleRarity(toggle, 4)}
        )
        .spacing(5);

        rarity_row
            .spacing(10)
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
