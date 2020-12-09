use iced::{
    Container, Command, Application, Element, Column, Length, Text, Align
};
use sqlx::SqlitePool;
use std::rc::Rc;
use crate::components::CardView;
use crate::db;

pub struct App {
    db: db::TDDatabase,
    state: AppState
}

#[derive(Debug, Clone)]
enum AppState {
    Error { error: db::Error },
    CardLoading,
    CardFound { card: CardView }
}

#[derive(Debug, Clone)]
pub enum Message {
    DbLoaded(Result<(), db::Error>),
    CardLoaded(Result<CardView, db::Error>),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let tddb = db::TDDatabase::new("sqlite:ayaya.db").unwrap();
        let td_clone = tddb.clone();
        (
            App {
                db: tddb,
                state: AppState::CardLoading
            },
            Command::perform(td_clone.init(), Message::DbLoaded)
        )
    }

    fn title(&self) -> String {
        String::from("icedPrincess")
    }
    
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CardLoaded(Ok(card)) => {
                self.state = AppState::CardFound{card};

                Command::none()
            }
            Message::CardLoaded(Err(error)) => {
                self.state = AppState::Error{error};

                Command::none()
            }
            Message::DbLoaded(Ok(_)) => match &self.state {
                AppState::CardLoading => {
                    self.state = AppState::CardLoading;

                    Command::perform(CardView::new(828, self.db.clone()), Message::CardLoaded)
                },
                _ => Command::none()
            }
            Message::DbLoaded(Err(error)) => {
                self.state = AppState::Error{error};
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match &self.state {
            AppState::CardLoading => Column::new()
                .width(Length::Shrink)
                .push(Text::new("Building and Getting Database").size(40)),
            AppState::CardFound { card } => Column::new()
                .align_items(Align::End)
                .push(card.view()),
            AppState::Error{ error } => Column::new()
                .push(Text::new("die").size(40))
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
