use iced::{Column, Element, Align, Text, 
    image, Image, button, Button, 
    Row, Length, scrollable, Scrollable,
};

use crate::{db, princess};
use crate::app::Message;

// TODO: Make level be inputtable

#[derive(Debug, Clone)]
pub enum CardMessage {
    IncreaseLevel,
    DecreaseLevel
}

#[derive(Debug, Clone)]
pub struct CardView {
    pub card: db::DbCard,
    current_vocal: i32,
    current_dance: i32,
    current_visual: i32,
    bg: image::Handle,
    card_art: image::Handle,
    back_button: button::State,
    increase_level: button::State,
    decrease_level: button::State,
    current_level: i32,
    scroll: scrollable::State
}

impl CardView {
    pub fn new(card: db::DbCard,
        bg: image::Handle, card_art: image::Handle) -> CardView {
        CardView {
            card,
            current_vocal: 0,
            current_dance: 0,
            current_visual: 0,
            bg,
            card_art,
            increase_level: button::State::new(),
            decrease_level: button::State::new(),
            current_level: 1,
            back_button: button::State::new(),
            scroll: scrollable::State::new()
        }
    }

    pub fn update(&mut self, message: CardMessage) {
        match message {
            CardMessage::IncreaseLevel => {
                self.current_level += 1;
            }
            CardMessage::DecreaseLevel => {
                self.current_level -= 1;
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        self.calc_level(self.current_level);
        let mut increase_level = Button::new(
            &mut self.increase_level, Text::new("+")
        );

        increase_level = if self.current_level != 90 {
            increase_level.on_press(
                Message::CardUpdate(CardMessage::IncreaseLevel)
            )
        } else {
            increase_level
        };

        let mut decrease_level = Button::new(
            &mut self.decrease_level, Text::new("-")
        );

        decrease_level = if self.current_level != 1 {
            decrease_level.on_press(
                Message::CardUpdate(CardMessage::DecreaseLevel)
            )
        } else {
            decrease_level
        };

        let values = Column::new()
            .push(
                Row::new()
                    .push(decrease_level)
                    .push(Text::new(self.current_level.to_string()))
                    .push(increase_level)
            )
            .push(
                Text::new(
                    format!("Vocal: {}", self.current_vocal)
                )
            )
            .push(
                Text::new(
                    format!("Dance: {}", self.current_dance)
                )
            )
            .push(
                Text::new(
                    format!("Visual: {}", self.current_visual)
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
                    format!("Center Skill: {}", princess::tl_center_skill(&self.card))
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

    fn calc_level(&mut self, level: i32) {
        self.current_vocal = CardView::interpolate(
            level, 1, 90,
            self.card.vocal_min_awakened, self.card.vocal_max_awakened
        );
        self.current_dance = CardView::interpolate(
            level, 1, 90, 
            self.card.dance_min_awakened, self.card.dance_max_awakened
        );
        self.current_visual = CardView::interpolate(
            level, 1, 90, 
            self.card.visual_min_awakened, self.card.visual_max_awakened
        );
    }

    fn interpolate(xp: i32, x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
        ((y0 as f32) + ((y1-y0) as f32 / (x1-x0) as f32) * ((xp - x0) as f32)) as i32
    }
}