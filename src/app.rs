use iced::{
    Container, Command, Application, Element, Column, Length, Text, Align
};

use crate::components::{CardView, CardListPage};
use crate::db;

pub struct App {
    db: db::TDDatabase,
    state: AppState,
    card_list: CardListPage,
    offset: i32,
    rarity_filter: Vec<i32>,
    filter: String,
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
    ToggleNormalRarity(bool),
    ToggleRareRarity(bool),
    ToggleSrRarity(bool),
    ToggleSsrRarity(bool),
    NextPage,
    PreviousPage
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
                offset: 0,
                rarity_filter: vec![1,2,3,4],
                filter: String::from("(1,2,3,4)")
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
                        self.filter.clone()), 
                        Message::CardsListed)
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
            Message::NextPage => {
                // TODO: Do better pagination, very flawed rn
                self.offset += 25;

                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset, 
                    self.filter.clone()), 
                    Message::CardsListed)
            }
            Message::PreviousPage => {
                self.offset -= 25;

                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset,
                    self.filter.clone()),
                    Message::CardsListed)
            }
            Message::ToggleNormalRarity(toggle) => {
                if toggle == false {
                    self.rarity_filter.retain(|&x| x != 1);
                    self.card_list.filter.n_toggle = false
                } else {
                    self.rarity_filter.push(1);
                    self.card_list.filter.n_toggle = true

                };
                self.offset = 0;
                self.filter = Self::construct_filter(self.rarity_filter.clone());

                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset,
                    self.filter.clone()),
                    Message::CardsListed)
            }
            Message::ToggleRareRarity(toggle) => {
                if toggle == false {
                    self.rarity_filter.retain(|&x| x != 2);
                    self.card_list.filter.r_toggle = false
                } else {
                    self.rarity_filter.push(2);
                    self.card_list.filter.r_toggle = true
                };
                self.offset = 0;
                self.filter = Self::construct_filter(self.rarity_filter.clone());

                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset, 
                    self.filter.clone()),
                    Message::CardsListed)
            }
            Message::ToggleSrRarity(toggle) => {
                if toggle == false {
                    self.rarity_filter.retain(|&x| x != 3);
                    self.card_list.filter.sr_toggle = false
                } else {
                    self.rarity_filter.push(3);
                    self.card_list.filter.sr_toggle = true
                };
                self.offset = 0;
                self.filter = Self::construct_filter(self.rarity_filter.clone());

                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset,
                    self.filter.clone()),
                    Message::CardsListed)
            }
            Message::ToggleSsrRarity(toggle) => {
                if toggle == false {
                    self.rarity_filter.retain(|&x| x != 4);
                    self.card_list.filter.ssr_toggle = false
                } else {
                    self.rarity_filter.push(4);
                    self.card_list.filter.ssr_toggle = true
                };
                self.offset = 0;
                self.filter = Self::construct_filter(self.rarity_filter.clone());

                Command::perform(
                    self.db.clone().get_card_list(
                        self.card_list.clone(),
                        self.offset,
                        self.filter.clone()),
                        Message::CardsListed)
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
                self.card_list = cards.as_ref().unwrap().clone(); 
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
