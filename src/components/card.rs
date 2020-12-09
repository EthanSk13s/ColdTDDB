use iced::{button, Column, Command, Element, Align, Text};

use crate::db;
use crate::app::Message;

#[derive(Debug, Clone)]
pub struct CardView {
    name: String,
    skill: String,
    center_skill: String,
    min_vocal: i32,
    max_vocal: i32,
    min_dance: i32,
    max_dance: i32,
    min_visual: i32,
    max_visual: i32
}

impl CardView {
    pub async fn new(id: i32, db: db::TDDatabase) -> Result<CardView, db::Error> {
        let card = db.get_card(id).await.unwrap();

        Ok(CardView {
            name: card.name.clone(),
            skill: card.skill.clone(),
            center_skill: card.center_skill.clone(),
            min_vocal: card.vocal_min,
            max_vocal: card.vocal_max,
            min_dance: card.dance_min,
            max_dance: card.dance_max,
            min_visual: card.visual_min,
            max_visual: card.visual_max
        })
    }

    pub fn view(&self) -> Element<Message> {
        Column::new()
            .padding(10)
            .align_items(Align::Center)
            .push(Text::new(&self.name))
            .push(Text::new(&self.max_vocal.to_string()))
            .push(Text::new(&self.max_dance.to_string()))
            .push(Text::new(&self.max_visual.to_string()))
            .push(Text::new(&self.skill))
            .push(Text::new(&self.center_skill))
            .into()
    }

}