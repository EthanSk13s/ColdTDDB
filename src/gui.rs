/* use serde::Deserialize;
use iced::{Element, Column, Align, Row, Text, Length, text_input, Command,
Application, Container, Button, button};

use sqlx::{SqlitePool};
use super::db;

#[derive(Debug, Clone)]
pub struct Card {
    pub card_id: i32,
    name: String,
    vocal: i32,
    dance: i32,
    visual: i32
}

#[derive(Debug, Clone)]
pub enum Message {
    CardFound(Result<Card, Error>),
    DbBuilt(Result<(), Error>),
    Search
}

pub enum IcedPrincess {
    Loading,
    Loaded {
        card: Card,
        search: button::State,
        input_value: String
    },
    Errored {
        error: Error,
        try_again: text_input::State,
    }
}

impl Application for IcedPrincess {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (IcedPrincess, Command<Self::Message>) {
        (
            IcedPrincess::Loading,
            Command::perform(Card::build_db(), Message::DbBuilt),
        )
    }

    fn title(&self) -> String {
        String::from("Iced-Princess")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CardFound(Ok(card)) => {
                *self = IcedPrincess::Loaded {
                    card,
                    search: button::State::new(),
                    input_value: String::from("me")
                };

                Command::none()
            },
            Message::CardFound(Err(error)) => {
                *self = IcedPrincess::Errored {
                    error,
                    try_again: text_input::State::new()
                };

                Command::none()
            },
            Message::DbBuilt(Ok(())) => match message {
                Message::CardFound(Ok(card)) => {
                    *self = IcedPrincess::Loaded {
                        card,
                        search: button::State::new(),
                        input_value: String::from("men")
                    };

                    Command::none()
                },
                _ => { match self {
                        IcedPrincess::Loading => Command::none(),
                        _ => {
                        *self = IcedPrincess::Loading;

                        Command::perform(Card::fetch_data(828), Message::CardFound)
                        }
                    }
                }
            },
            Message::DbBuilt(Err(error)) => {
                *self = IcedPrincess::Errored {
                    error,
                    try_again: text_input::State::new()
                };

                Command::none()
            }
            Message::Search => match self {
                IcedPrincess::Loading => Command::none(),
                _ => {
                    *self = IcedPrincess::Loading;

                    Command::perform(Card::fetch_data(828), Message::CardFound)
                }
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match self {
            IcedPrincess::Loading => Column::new()
                .width(Length::Shrink)
                .push(Text::new("Searching for Cutieha...").size(30)),
            IcedPrincess::Loaded { card, search, input_value } => Column::new()
                .max_width(300)
                .spacing(20)
                .align_items(Align::End)
                .push(card.view())
                .push(
                    button(search, "Keep searching!").on_press(Message::Search),
                ),
            IcedPrincess::Errored {try_again, ..} => Column::new()
                .spacing(20)
                .align_items(Align::End)
                .push(Text::new("meh").size(40))
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl Card {
    fn view(&mut self) -> Element<Message> {
        Column::new()
            .spacing(20)
            .align_items(Align::End)
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(
                        Text::new(&self.name)
                            .size(25)
                            .width(Length::Fill),
                    )
                    .push(
                        Text::new(format!("{}", self.card_id))
                            .size(20)
                    ),
            )
            .push(Text::new(format!("Vocal: {}", self.vocal)))
            .into()
    }

    pub async fn fetch_data(id: u32) -> Result<Card, Error> {

        let pool = SqlitePool::connect("sqlite:ayaya.db").await?;
        let test = db::TDDatabase{pool: pool};

        let card = test.get_card(828).await.unwrap();

        //db::add_card(&pool, card).await?;

        Ok(Card {
            card_id: card.card_id,
            name: card.name.clone(),
            vocal: card.vocal_min,
            dance: card.dance_min,
            visual: card.visual_min
        })
    }

    pub async fn build_db() -> Result<(), Error> {
        let pool = SqlitePool::connect("sqlite:ayaya.db").await?;
        let test = db::TDDatabase{pool: pool};

        test.create_tables().await.expect("OOPSIES");
        test.init().await.expect("OOPS");

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    APIError,
    LanguageError,
    SqlxError
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Error {
        dbg!(error);

        Error::SqlxError
    }
}

fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
        .style(style::Button::Primary)
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
*/