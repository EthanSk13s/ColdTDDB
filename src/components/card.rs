use iced::{Column, Element, Align, Text, 
    image, Image, button, Button, 
    Row, Length, scrollable, Scrollable
};

use crate::{db, princess};
use crate::app::Message;

#[derive(Debug, Clone)]
pub struct CardView {
    card: db::DbCard,
    bg: image::Handle,
    card_art: image::Handle,
    back_button: button::State,
    scroll: scrollable::State
}

impl CardView {
    pub fn new(card: db::DbCard,
        bg: image::Handle, card_art: image::Handle) -> CardView {
        CardView {
            card,
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
                    format!("Vocal: {}", &self.card.vocal_max_awakened)
                )
            )
            .push(
                Text::new(
                    format!("Dance: {}", &self.card.dance_max_awakened)
                )
            )
            .push(
                Text::new(
                    format!("Visual: {}", &self.card.visual_max_awakened)
                )
            )
            .push(
                Text::new(
                    format!("Skill Type: {}", princess::match_skill_type(self.card.effect))
                )
            )
            .push(
                Text::new(
                    format!("Skill: {}", princess::tl_skill(&self.card))
                )
            )
            .push(
                Text::new(
                    format!("Center Skill: {}", &self.card.center_skill)
                )
            )
            .padding(5)
            .width(Length::FillPortion(2));

        let info = Row::new()
            .push(
                Image::new(self.card_art.clone())
                    .width(Length::Units(320))
                    .height(Length::Units(400))
            )
            .width(Length::FillPortion(1))
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
                Text::new(&self.card.name)
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