use iced::{
    button, Button, Column, Element,
    Align, Text, Row, scrollable, Scrollable,
    Length, image, Image, Space
};

use super::filters::{RarityFilter, TypeFilter, IdolFilter, SkillFilter};
use crate::db;
use crate::app::Message;
use crate::styles;
use crate::princess;

#[derive(Debug, Clone)]
pub enum FilterMessage {
    ToggleRarity(bool, i32),
    ToggleType(bool, i32),
    ToggleSkill(bool, i16),
}

#[derive(Debug, Clone)]
pub struct CardButton {
    link: button::State,
    icon: image::Handle,
    pub card: db::DbCard
}

impl CardButton {
    pub fn new(icon: image::Handle, card: db::DbCard) -> CardButton {
        CardButton {
            link: button::State::new(),
            icon,
            card
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let mini_info = Row::new()
            .push(
                Column::new()
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(
                                Text::new(
                                    format!(
                                        "Vo: {}", 
                                        self.card.vocal_max_awakened
                                    )
                                ).size(18)
                            )
                            .push(
                                Text::new(
                                    format!(
                                        "Da: {}",
                                        self.card.dance_max_awakened
                                    )
                                ).size(18)
                            )
                            .push(
                                Text::new(
                                    format!(
                                        "Vi: {}",
                                        self.card.visual_max_awakened
                                    )
                                ).size(18)
                            )
                    ).push(
                        Row::new()
                            .push(
                                Text::new(
                                    format!(
                                        "Skill: {}",
                                        princess::match_skill_type(self.card.skill_id)
                                    )
                                )
                            )
                    )
            );

        let content = Row::with_children(vec![
            Row::new().push(Image::new(self.icon.clone())
                .width(Length::Units(50))
                .height(Length::Units(50)))
                .push(
                    Text::new(self.card.name.to_owned())
                    .size(30)
                ).align_items(Align::Center)
                .width(Length::FillPortion(2)).into(),
            Space::with_width(Length::FillPortion(2)).into(),
            mini_info.into(),
            Space::with_width(Length::FillPortion(2)).into()
        ]);

        let link_button = Button::new(&mut self.link, content)
            .on_press(Message::CardPressed(self.card.card_id))
            .style(styles::CardButtonStyle {
                idol_type: self.card.idol_type
            })
            .width(Length::Fill);

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
    pub idol_filter: IdolFilter,
    pub skill_filter: SkillFilter
}

impl ListFilters {
    pub fn new() -> ListFilters { 
        ListFilters {
            rarity_filter: RarityFilter::new(),
            type_filter: TypeFilter::new(),
            idol_filter: IdolFilter::new(),
            skill_filter: SkillFilter::new()
        }
    }

    pub fn update(&mut self, message: FilterMessage) {
        match message {
            FilterMessage::ToggleRarity(toggle, rarity) => {
                if toggle == false {
                    self.rarity_filter.current_filters
                        .retain(|&x| x != rarity);
                    self.rarity_filter.set_state(rarity, toggle)
                } else {
                    self.rarity_filter.current_filters.push(rarity);
                    self.rarity_filter.set_state(rarity, toggle)
                };
            }
            FilterMessage::ToggleType(toggle, idol_type) => {
                if toggle == false {
                    self.type_filter.current_filters
                        .retain(|&x| x != idol_type);
                    self.type_filter.set_state(idol_type, toggle)
                } else {
                    self.type_filter.current_filters.push(idol_type);
                    self.type_filter.set_state(idol_type, toggle)
                };                
            }
            FilterMessage::ToggleSkill(toggle, skill_type) => {
                if toggle == false {
                    self.skill_filter.current_filters
                        .retain(|&x| x != skill_type as i32);
                    self.skill_filter.set_state(skill_type, toggle)
                } else {
                    self.skill_filter.current_filters.push(skill_type.into());
                    self.skill_filter.set_state(skill_type, toggle)
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let filter_column = Column::new();
        let first_row = Row::new();
        let second_row = Row::new();

        filter_column
            .push(
                first_row
                    .spacing(10)
                    .push(self.idol_filter.view())
                    .push(self.type_filter.view())
                    .push(self.rarity_filter.view())
            )
            .push(
                second_row.spacing(10)
                    .push(self.skill_filter.view())
            )
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
        let first_card = self.cards[0].card.card_id;
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