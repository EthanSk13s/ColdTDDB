use iced::{
    Container, Command, Application, Element, Column, Length, Text, image
};

use crate::components::{CardView, CardListPage};
use crate::db;

pub struct App {
    db: db::TDDatabase,
    state: AppState,
    card_list: CardListPage,
    current_card: CardView,
    offset: i32,
    previous_offsets: Vec<i32>,
    rarity_filter: Vec<i32>,
    filter: String,
    min: i32
}

#[derive(Debug, Clone)]
enum AppState {
    Error { error: db::Error },
    CardLoading,
    CardFound { card: CardView },
    CardList { cards: Result<CardListPage, db::Error> }
}

#[derive(Debug, Clone)]
pub enum Message {
    DbLoaded(Result<(), db::Error>),
    CardLoaded(Result<CardView, db::Error>),
    CardPressed(i32),
    CardsListed(Result<CardListPage, db::Error>),
    ToggleRarity(bool, i32),
    NextPage,
    PreviousPage,
    ReturnToList,
}

impl App {
    fn construct_filter(rarity: Vec<i32>) -> String {
        let mut filter = String::from("(");
        for v in rarity {
            let query = format!(",{}", &v.to_string().to_owned());
            filter.push_str(&query)
        }

        filter.remove(1);
        filter.push_str(")");
        filter
    }
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
                card_list: CardListPage::new(0).unwrap(),
                current_card: CardView::new(Default::default(), image::Handle::from("")),
                offset: 0,
                previous_offsets: vec![1],
                rarity_filter: vec![1,2,3,4],
                filter: String::from("(1,2,3,4)"),
                min: 0
            },
            Command::perform(td_clone.init(), Message::DbLoaded)
        )
    }

    fn title(&self) -> String {
        String::from("ColdTDDB")
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

                    Command::perform(self.db.clone().get_card_list(
                        self.card_list.clone(), 
                        self.offset, 
                        self.filter.to_owned()), 
                        Message::CardsListed)
                },
                _ => Command::none()
            }
            Message::DbLoaded(Err(error)) => {
                self.state = AppState::Error{error};
                Command::none()
            }
            Message::CardPressed(id) => {
                Command::perform(self.db.clone().get_card(id), Message::CardLoaded)
            }
            Message::CardsListed(cards) => {
                let min = self.previous_offsets.get(0);
                self.min = match min {
                    Some(id) => *id,
                    None => cards.clone().unwrap().cards[0].id
                };

                self.state = AppState::CardList{cards};

                Command::none()
            }
            Message::NextPage => {
                self.offset = self.card_list.cards[24].id;
                self.previous_offsets.push(self.card_list.cards[0].id);

                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset, 
                    self.filter.to_owned()), 
                    Message::CardsListed)
            }
            Message::PreviousPage => {
                let value = self.previous_offsets.pop();
                self.offset = match value {
                    Some(id) => id - 1,
                    None => 0
                };

                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset,
                    self.filter.to_owned()),
                    Message::CardsListed)
            }
            Message::ToggleRarity(toggle, rarity) => {
                if toggle == false {
                    self.rarity_filter.retain(|&x| x != rarity);
                    self.card_list.filter.set_state(rarity, toggle)
                } else {
                    self.rarity_filter.push(rarity);
                    self.card_list.filter.set_state(rarity, toggle)
                };

                self.offset = 0;
                self.min = 0;
                self.filter = Self::construct_filter(self.rarity_filter.clone());
                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset,
                    self.filter.to_owned()),
                    Message::CardsListed)
            }
            Message::ReturnToList => {
                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset,
                    self.filter.to_owned()),
                    Message::CardsListed)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match &self.state {
            AppState::CardLoading => Column::new()
                .width(Length::Shrink)
                .push(Text::new("Building and Getting Database").size(40)),
            AppState::CardFound { card } => { 
                self.current_card = card.clone();
                Column::new()
                .push(self.current_card.view())
            },
            AppState::Error{ error } => Column::new()
                .push(Text::new("die").size(40)),
            AppState::CardList { cards } => {
                self.card_list = cards.as_ref().unwrap().clone();
                self.card_list.set_min(self.min); 
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
