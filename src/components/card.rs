use iced::{Column, Element, Align, Text, 
    image, Image, button, Button, 
    Row, Length, scrollable, Scrollable};

use crate::db;
use crate::app::Message;

#[derive(Debug, Clone)]
pub struct CardView {
    name: String,
    rarity: i32,
    skill: String,
    center_skill: String,
    min_vocal: i32,
    max_vocal: i32,
    min_dance: i32,
    max_dance: i32,
    min_visual: i32,
    max_visual: i32,
    bg: image::Handle,
    card_art: image::Handle,
    back_button: button::State,
    scroll: scrollable::State
}

impl CardView {
    pub fn new(card: db::DbCard,
        bg: image::Handle, card_art: image::Handle) -> CardView {
        CardView {
            name: card.name.clone(),
            rarity: card.rarity,
            skill: card.skill.clone(),
            center_skill: card.center_skill.clone(),
            min_vocal: card.vocal_min,
            max_vocal: card.vocal_max,
            min_dance: card.dance_min,
            max_dance: card.dance_max,
            min_visual: card.visual_min,
            max_visual: card.visual_max,
            bg,
            card_art,
            back_button: button::State::new(),
            scroll: scrollable::State::new()
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let values = Column::new()
            .push(
                Text::new(
                    format!("Vocal: {}", &self.max_vocal.to_string())
                )
            )
            .push(
                Text::new(
                    format!("Dance: {}", &self.max_dance.to_string())
                )
            )
            .push(
                Text::new(
                    format!("Visual: {}", &self.max_visual.to_string())
                )
            )
            .push(
                Text::new(
                    format!("Skill: {}", &self.skill)
                )
            )
            .push(
                Text::new(
                    format!("Center Skill: {}", &self.center_skill)
                )
            )
            .padding(5)
            .width(Length::FillPortion(2));

        let info = Row::new()
            .push(
                Image::new(self.card_art.clone())
                    .width(Length::FillPortion(1))
                    .height(Length::FillPortion(1))
            )
            .push(values);

        let card_bg = Row::new()
            .push(
                Image::new(self.bg.clone())
                    .width(Length::Units(640))
                    .height(Length::Units(360))
            )
            .width(Length::Fill);

        let content = Scrollable::new(&mut self.scroll)
            .height(Length::Fill)
            .width(Length::Fill)
            .push(
                Text::new(&self.name)
                    .size(42)
            )
            .push(info)
            .spacing(5)
            .push(card_bg);

        let back = Button::new(&mut self.back_button, Text::new("Back"))
            .on_press(Message::ReturnToList);
        
        Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(10)
            .align_items(Align::Start)
            .push(content)
            .push(
                Row::new()
                    .push(back)
                    .align_items(Align::Start)
            )
            .into()
    }
}