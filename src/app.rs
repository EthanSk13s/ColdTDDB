use iced::{
    Container, Command, Application, Element, Column, Length, Text, image
};

use crate::components::{
    CardView, CardListPage, IdolList,
    FilterMessage, CardMessage
};
use crate::db;

pub struct App {
    db: db::TDDatabase,
    state: AppState,
    card_list: CardListPage,
    current_card: CardView,
    offset: i32,
    previous_offsets: Vec<i32>,
    idol_filter: i32,
    filter: String,
    min: i32
}

#[derive(Debug, Clone)]
enum AppState {
    Error { error: db::Error },
    CardLoading,
    CardFound { card: CardView },
    CardList { cards: Result<CardListPage, db::Error> },
    CardListNotFound { cards: Result<CardListPage, db::Error> },
}

#[derive(Debug, Clone)]
pub enum Message {
    DbLoaded(Result<(), db::Error>),
    CardLoaded(Result<CardView, db::Error>),
    CardPressed(i32),
    CardsListed(Result<CardListPage, db::Error>),
    FilterUpdate(FilterMessage),
    CardUpdate(CardMessage),
    PickIdol(IdolList),
    NextPage,
    PreviousPage,
    ReturnToList,
}

impl App {
    fn construct_filter(&mut self) {
        let mut rarity_filter = String::from("rarity IN (");

        rarity_filter = App::check_len(
            &self.card_list.filter.rarity_filter.current_filters,
            &mut rarity_filter,
            11
        );

        if self.card_list.filter.rarity_filter.current_filters.len() == 0 {
            rarity_filter.push_str("1,2,3,4)");
            
            for x in vec![1,2,3,4] {
                self.card_list.filter.rarity_filter
                    .set_state(x, true);

                self.card_list.filter.
                    rarity_filter.current_filters.push(x)
            }
        }

        let mut type_filter = String::from("idol_type IN (");

        type_filter = App::check_len(
            &self.card_list.filter.type_filter.current_filters, 
            &mut type_filter, 
            14
        );

        if self.card_list.filter.type_filter.current_filters.len() == 0 {
            type_filter.push_str("1,2,3,5)");

            for x in vec![1,2,3,5] {
                self.card_list.filter.type_filter
                    .set_state(x, true);

                self.card_list.filter
                    .type_filter.current_filters.push(x)
            }
        }

        let mut skill_filter = String::from("skills.effect IN (");

        skill_filter = App::check_len(
            &self.card_list.filter.skill_filter.current_filters,
            &mut skill_filter,
            18
        );

        if self.card_list.filter.skill_filter.current_filters.len() == 0 {
            skill_filter.push_str("1,2,3,4,5,6,7,8,10,11)");

            for x in vec![1,2,3,4,5,6,7,8,10,11] {
                self.card_list.filter.skill_filter
                    .set_state(x, true);

                self.card_list.filter
                    .skill_filter.current_filters.push(x.into())
            }
        }

        let mut idol_filter = String::from("");
        if self.idol_filter != 0 {
            idol_filter = format!("AND idol_id == {}", self.idol_filter);
        }

        let query = format!(
            "{} AND {} AND {} {}",
            rarity_filter, type_filter, 
            skill_filter, idol_filter
        );

        self.filter = query;
    }

    fn check_len(x: &Vec<i32>, y: &mut String, i: usize) -> String {
        if x.len() != 0 {
            for v in x.iter() {
                let query = format!(",{}", &v.to_string().to_owned());
                y.push_str(&query)
            }

            y.remove(i);
            y.push_str(")");
        };

        y.to_string()
    }
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let tddb = db::TDDatabase::new("sqlite:cache/td.db").unwrap();
        let td_clone = tddb.clone();
        (
            App {
                db: tddb,
                state: AppState::CardLoading,
                card_list: CardListPage::new(0).unwrap(),
                current_card: CardView::new(
                    Default::default(),
                    image::Handle::from(""),
                    image::Handle::from("")
                ),
                offset: 0,
                previous_offsets: vec![1],
                idol_filter: 0,
                filter: String::from(
                    r#"rarity in (1,2,3,4)
                    AND idol_type in (1,2,3,5)
                    "#
                ),
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
                if cards.clone().unwrap().cards.len() != 0 {
                    let min = self.previous_offsets.get(0);
                    self.min = match min {
                        Some(id) => *id,
                        None => cards.clone().unwrap().cards[0].card.card_id
                    };

                    self.state = AppState::CardList{cards}
                } else {
                    self.state = AppState::CardListNotFound{cards}
                };

                Command::none()
            }
            Message::NextPage => {
                self.offset = self.card_list.cards[24].card.card_id;
                self.previous_offsets.push(self.card_list.cards[0].card.card_id);

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
            Message::ReturnToList => {
                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset,
                    self.filter.to_owned()),
                    Message::CardsListed)
            }
            Message::PickIdol(idol) => {
                self.idol_filter = idol as i32;
                self.card_list.filter.idol_filter.selected = idol;

                self.offset = 0;
                self.min = 0;
                self.construct_filter();
                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset,
                    self.filter.to_owned()),
                    Message::CardsListed)
            }
            Message::FilterUpdate(filter_message) => {
                self.card_list.filter.update(filter_message);

                self.offset = 0;
                self.min = 0;
                self.construct_filter();
                Command::perform(self.db.clone().get_card_list(
                    self.card_list.clone(),
                    self.offset,
                    self.filter.to_owned()),
                    Message::CardsListed)
            }
            Message::CardUpdate(card_message) => {
                self.current_card.update(card_message);
    
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match &self.state {
            AppState::CardLoading => Column::new()
                .width(Length::Shrink)
                .push(Text::new("Building and Getting Database").size(40)),
            AppState::CardFound { card } => {
                if self.current_card.card.id != card.card.id { 
                    self.current_card = card.clone();
                }
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
            AppState::CardListNotFound { cards } => {
                self.card_list = cards.as_ref().unwrap().clone();
                Column::new()
                    .push(self.card_list.empty_view())
            }
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
