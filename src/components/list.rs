use iced::{button, Button, Column, Element, Align, Text, Row, scrollable, Scrollable};

use crate::db;
use crate::app::Message;

#[derive(Debug, Clone)]
pub struct CardButton {
    name: String,
    id: i32,
    link: button::State
}

impl CardButton {
    pub fn new(id: i32, name: String) -> CardButton {
        CardButton {
            name: name,
            id: id,
            link: button::State::new()
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let content = Row::new().push(Text::new(self.name.to_owned()));
        let link_button = Button::new(&mut self.link, content)
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
    next_button: button::State,
    previous_button: button::State,
    offset: i32
}

impl CardListPage {
    pub fn new(offset: i32) -> Result<CardListPage, db::Error> {
        let card_buttons = vec![];

        Ok(CardListPage {
            state: scrollable::State::new(),
            cards: card_buttons,
            next_button: button::State::new(),
            previous_button: button::State::new(),
            offset
        })
    }

    pub fn get_buttons(&mut self, cards: Vec<CardButton>) {
        self.cards = cards;
    }

    pub fn view<'a>(&'a mut self) -> Element<Message> {
        let column = Column::new();
        let mut content = Scrollable::new(&mut self.state);
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
        
        previous_button = if self.offset != 0 {
            previous_button.on_press(Message::PreviousPage)
        } else {
            previous_button
        };

        let page_controls = Row::new().push(previous_button).push(next_button);

        content = content.push(page_controls);

        column.push(content).into()
    }
}