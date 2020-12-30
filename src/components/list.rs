use iced::{
    button, Button, Column, Element,
    Align, Text, Row, scrollable, Scrollable,
    Length, image, Image
};

use super::filters::{RarityFilter, TypeFilter, IdolFilter};
use crate::db;
use crate::app::Message;
use crate::styles;

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
            .on_press(Message::CardPressed(self.id))
            .style(styles::CardButtonStyle);

        Row::new()
            .align_items(Align::Start)
            .push(link_button)
            .into()
    }
}
#[derive(Debug, Clone)]
pub struct ListFilters {
    pub rarity_filter: RarityFilter,
    pub type_filter: TypeFilter,
    pub idol_filter: IdolFilter
}

impl ListFilters {
    pub fn new() -> ListFilters { 
        ListFilters {
            rarity_filter: RarityFilter::new(),
            type_filter: TypeFilter::new(),
            idol_filter: IdolFilter::new()
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        let rarity_row = Row::new();

        rarity_row
            .spacing(10)
            .push(self.idol_filter.view())
            .push(self.type_filter.view())
            .push(self.rarity_filter.view())
            .into()

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

    pub fn empty_view(&mut self) -> Element<Message> {
        let column = Column::new();
        
        column
            .width(Length::Fill)
            .height(Length::Fill)
            .push(self.filter.view())
            .push(
                Text::new("No cards found.")
                    .size(54)
            )
            .align_items(Align::Center)
            .into()
    }
}