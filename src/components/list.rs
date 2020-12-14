use iced::{button, Button, Column, Element, Align, Text, Row, scrollable, Scrollable};

use crate::db;
use crate::app::Message;

#[derive(Debug, Clone, Copy)]
pub struct CardButton {
    id: i32,
    link: button::State
}

impl CardButton {
    pub fn new(id: i32) -> CardButton {
        CardButton { 
            id: id,
            link: button::State::new()
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let link_button = Button::new(&mut self.link, Text::new(self.id.to_string()))
            .on_press(Message::CardPressed(self.id));

        Row::new()
            .align_items(Align::Start)
            .push(link_button)
            .into()
    }
}

#[derive(Debug, Clone)]
pub struct CardListPage {
    state: scrollable::State,
    cards: Vec<CardButton>,
}

impl CardListPage {
    pub fn new() -> Result<CardListPage, db::Error> {
        let card_buttons = vec![];

        Ok(CardListPage {
            state: scrollable::State::new(),
            cards: card_buttons
        })
    }

    pub fn get_buttons(&mut self, cards: Vec<CardButton>) {
        self.cards = cards;
    }

    pub fn view<'a>(&'a mut self) -> Element<Message> {
        let column = Column::new();
        let mut content = Scrollable::new(&mut self.state);

        for card in self.cards.iter_mut() {
            content = content.push(card.view());
        }

        column.push(content).into()
    }
}