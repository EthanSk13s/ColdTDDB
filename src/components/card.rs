use iced::{Column, Element, Align, Text, 
    image, Image, button, Button};

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
    back_button: button::State
}

impl CardView {
    pub fn new(card: db::DbCard, bg: image::Handle) -> CardView {
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
            back_button: button::State::new()
        }
    }

    pub fn view<'a>(&'a mut self) -> Element<Message> {
        let back = Button::new(&mut self.back_button, Text::new("Back"))
            .on_press(Message::ReturnToList);
        
        Column::new()
            .align_items(Align::Start)
            .push(back)
            .padding(10)
            .align_items(Align::Center)
            .push(Text::new(&self.name))
            .push(Text::new(&self.max_vocal.to_string()))
            .push(Text::new(&self.max_dance.to_string()))
            .push(Text::new(&self.max_visual.to_string()))
            .push(Text::new(&self.skill))
            .push(Text::new(&self.center_skill))
            .push(Image::new(self.bg.clone()))
            .into()
    }
}