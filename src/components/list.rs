use iced::{
    button, Button, Column, Element,
    Align, Text, Row, scrollable, Scrollable,
    Length, image, Image, Checkbox
};

use crate::db;
use crate::app::Message;

#[derive(Debug, Clone)]
pub struct CardButton {
    name: String,
    pub id: i32,
    link: button::State,
    icon: image::Handle
}

impl CardButton {
    pub fn new(id: i32, name: String, icon: image::Handle) -> CardButton {
        CardButton {
            name,
            id,
            link: button::State::new(),
            icon
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let content = Row::new()
            .push(
                Image::new(self.icon.clone())
                .width(Length::Units(50))
                .height(Length::Units(50))
            )
            .push(
                Text::new(self.name.to_owned())
                .size(30)
            )
            .align_items(Align::Center);
        let link_button = Button::new(&mut self.link, content)
            .on_press(Message::CardPressed(self.id));

        Row::new()
            .align_items(Align::Start)
            .push(link_button)
            .into()
    }
}
#[derive(Debug, Clone)]
pub struct ListFilters {
    pub n_toggle: bool,
    pub r_toggle: bool,
    pub sr_toggle: bool,
    pub ssr_toggle: bool
}

impl ListFilters {
    pub fn new() -> ListFilters { 
        ListFilters {
            n_toggle: true,
            r_toggle: true,
            sr_toggle: true,
            ssr_toggle: true,
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

#[derive(Debug, Clone)]
pub struct CardListPage {
    state: scrollable::State,
    pub cards: Vec<CardButton>,
    next_button: button::State,
    previous_button: button::State,
    pub offset: i32,
    pub filter: ListFilters,
    pub min: i32
}

impl CardListPage {
    pub fn new(offset: i32) -> Result<CardListPage, db::Error> {
        let card_buttons = vec![];

        Ok(CardListPage {
            state: scrollable::State::new(),
            cards: card_buttons,
            next_button: button::State::new(),
            previous_button: button::State::new(),
            offset,
            filter: ListFilters::new(),
            min: 0
        })
    }

    pub fn get_buttons(&mut self, cards: Vec<CardButton>) {
        self.cards.clear();
        self.cards = cards;
    }

    pub fn set_min(&mut self, min: i32) {
        self.min = min;
    }

    pub fn view<'a>(&'a mut self) -> Element<Message> {
        let column = Column::new();
        let mut content = Scrollable::new(&mut self.state);
        let first_card = self.cards[0].id;
        let size = self.cards.len();

        for card in self.cards.iter_mut() {
            content = content.push(card.view());
        }

        let mut next_button = Button::new(&mut self.next_button, Text::new(">"));
        let mut previous_button = Button::new(&mut self.previous_button, Text::new("<"));

        next_button = if size > 24 {
            next_button.on_press(Message::NextPage)
        } else {
            next_button
        };

        previous_button = if first_card > self.min + 1 {
            previous_button.on_press(Message::PreviousPage)
        } else {
            previous_button
        };

        let page_controls = Row::new()
            .push(previous_button)
            .push(next_button)
            .spacing(10);

        content = content.width(Length::Fill).height(Length::Fill);
        column
            .push(self.filter.view())
            .push(content)
            .push(page_controls)
            .align_items(Align::Center)
            .into()
    }
}