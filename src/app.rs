use iced::{
    Container, Command, Application, Element, Column, Length, Text, Align
};

use crate::components::{CardView, CardListPage};
use crate::db;

pub struct App {
    db: db::TDDatabase,
    state: AppState,
    card_list: CardListPage,
}

#[derive(Debug, Clone)]
enum AppState {
    Error { error: db::Error },
    CardLoading,
    CardFound { card: CardView },
    CardList { cards: CardListPage}
}

#[derive(Debug, Clone)]
pub enum Message {
    DbLoaded(Result<(), db::Error>),
    CardLoaded(Result<CardView, db::Error>),
    CardPressed(i32),
    CardsListed(CardListPage)
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
                state: AppState::CardLoading,
                card_list: CardListPage::new().unwrap()
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

                    Command::perform(self.db.clone().get_card_list(), Message::CardsListed)
                },
                _ => Command::none()
            }
            Message::DbLoaded(Err(error)) => {
                self.state = AppState::Error{error};
                Command::none()
            }
            Message::CardPressed(id) => {
                Command::perform(CardView::new(id, self.db.clone()), Message::CardLoaded)
            }
            Message::CardsListed(cards) => {
                self.state = AppState::CardList{cards};

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
                .push(Text::new("die").size(40)),
            AppState::CardList { cards } => {
                self.card_list = cards.clone(); 
                Column::new()
                .push(self.card_list.view())
            }
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
