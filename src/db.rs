use std::path::Path;

use sqlx::{SqlitePool};
use serde::Deserialize;
use iced::{image};
use crate::components::{CardListPage, CardButton, CardView};

use super::princess;

#[derive(Debug, Clone, Default, sqlx::FromRow)]
pub struct DbCard {
    id: i32,
    pub card_id: i32,
    idol_id: i32,
    pub name: String,
    pub rarity: i32,
    idol_type: i8,
    pub extra_type: i8,
    pub skill: String,
    pub center_skill: String,
    pub vocal_min: i32,
    pub dance_min: i32,
    pub visual_min: i32,
    pub vocal_max: i32,
    pub dance_max: i32,
    pub visual_max: i32,
    vocal_min_awakened: i32,
    dance_min_awakened: i32,
    visual_min_awakened: i32,
    vocal_max_awakened: i32,
    dance_max_awakened: i32,
    visual_max_awakened: i32,
    pub resource_id: String
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct JsonCard {
    id: i32,
    name: String,
    idol_id: i16,
    rarity: i16,
    idol_type: i16,
    extra_type: i16,
    #[serde(default="test")]
    pub skill: Vec<JsonSkill>,
    #[serde(default)]
    center_effect: JsonCenter,
    // release: String,
    vocal_min: i32,
    dance_min: i32,
    visual_min: i32,
    vocal_max: i32,
    dance_max: i32,
    visual_max: i32,
    vocal_min_awakened: i32,
    dance_min_awakened: i32,
    visual_min_awakened: i32,
    vocal_max_awakened: i32,
    dance_max_awakened: i32,
    visual_max_awakened: i32,
    resource_id: String
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct JsonSkill {
    id: i32,
    description: String,
    pub effect_id: i16,
    pub evaluation: i16,
    pub evaluation2: i16,
    pub duration: i16,
    pub interval: i16,
    pub probability: i16,
    pub value: Vec<i32>
}

fn test() -> Vec<JsonSkill> {
    let result = vec![JsonSkill {
        id: 0,
        description: String::from("null"),
        effect_id: 0,
        evaluation: 0,
        evaluation2: 0,
        duration: 0,
        interval: 0,
        probability: 0,
        value: vec![]
    }];
    result
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct JsonCenter {
    id: i32,
    pub description: String,
    pub idol_type: i16,
    pub attribute: i16,
    pub value: i32,
    #[serde(default="set_song")]
    pub song_type: i16,
    #[serde(default="set_value_2")]
    pub value_2: i32
}

fn set_song() -> i16 { 0 }
fn set_value_2() -> i32 { 0 }

impl Default for JsonCenter{
    fn default() -> Self {
        let result = JsonCenter {
            id: 0,
            description: String::from("null"),
            idol_type: 0,
            attribute: 0,
            value: 0,
            song_type: 0,
            value_2: 0
        };
        result
    }
}

#[derive(Clone)]
pub struct TDDatabase {
    pub pool: SqlitePool,
    pub limit: i32
}

impl TDDatabase {
    pub fn new(uri: &str) -> Result<Self, sqlx::Error> {
        Ok(TDDatabase {
            pool: SqlitePool::connect_lazy(&uri).unwrap(),
            limit: 0
        })
    }

    pub async fn create_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cards(
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                card_id INTEGER,
                rarity INTEGER,
                idol_id INTEGER,
                name TEXT,
                idol_type INTEGER,
                extra_type INTEGER,
                skill TEXT,
                center_skill TEXT,
                vocal_min INTEGER,
                dance_min INTEGER,
                visual_min INTEGER,
                vocal_max INTEGER,
                dance_max INTEGER,
                visual_max INTEGER,
                vocal_min_awakened INTEGER,
                dance_min_awakened INTEGER,
                visual_min_awakened INTEGER,
                vocal_max_awakened INTEGER,
                dance_max_awakened INTEGER,
                visual_max_awakened INTEGER,
                resource_id TEXT
            )
            "#
        ).execute(&self.pool).await?;
    
        Ok(())
    }
    
    pub async fn init(self) -> Result<(), Error> {
        if self.check_exists().await.unwrap() {
            self.create_tables().await;

            let data = reqwest::get("https://api.matsurihi.me/mltd/v1/cards/")
            .await?
            .text()
            .await?;

            let entry = serde_json::from_str::<Vec<JsonCard>>(&data).unwrap();
        
            for card in entry {
                self.add_card(card).await?;
            }
        }

        Ok(())
    }

    pub async fn get_card(self, card_id: i32) -> Result<CardView, Error> {
        let card = sqlx::query_as::<_, DbCard>(
            "SELECT * FROM cards WHERE card_id = $1"
        )
        .bind(card_id)
        .fetch_one(&self.pool).await?;

        let client = reqwest::Client::new();
        let bg = if card.rarity > 3 {
            let file_path = format!("cache/card_bg/{}.png", card.resource_id);
            let url = format!(
                "https://storage.matsurihi.me/mltd/card_bg/{}_1.png",
                card.resource_id
            );

            Self::handle_image(&client, file_path, url).await?
        } else {
            image::Handle::from("")
        };

        let card_art = {
            let file_path = format!("cache/card_art/{}.png", card.resource_id);
            let url = format!(
                "https://storage.matsurihi.me/mltd/card/{}_1_a.png",
                card.resource_id
            );

            Self::handle_image(&client, file_path, url).await?
        };

        let view = CardView::new(card, bg, card_art);
        Ok(view)
    }

    pub async fn get_card_list(
        self, current: CardListPage,
        offset: i32, filter: String) -> Result<CardListPage, Error> {
        let query = format!(
            r#"SELECT * FROM cards
            WHERE card_id > $1
            AND rarity IN {}
            ORDER BY card_id
            LIMIT 25
            "#, filter);

        let cards = sqlx::query_as::<_, DbCard>(&query)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .unwrap();

        let mut buttons = vec![];
        let client = reqwest::Client::new();
        for card in cards {
            let file_path = format!("cache/icons/{}.png", card.resource_id);
            let url = format!(
                "https://storage.matsurihi.me/mltd/icon_l/{}_1.png",
                card.resource_id
            );

            let icon = Self::handle_image(&client, file_path, url).await?;

            buttons.push(CardButton::new(card.card_id, card.name, icon));
        }

        let mut card_list = current;
        card_list.get_buttons(buttons);
        card_list.offset = offset;

        Ok(card_list)
    }

    async fn add_card(&self, card: JsonCard) -> Result<(), Error> {
        // This is so prone to SQL injections, but for sake of pracitce leave it be...
        sqlx::query(
            r#"INSERT INTO cards
        VALUES (
            null, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
            $21
        )"#)
            .bind(card.id)
            .bind(card.rarity)
            .bind(card.idol_id)
            .bind(princess::set_name(card.name))
            .bind(card.idol_type)
            .bind(card.extra_type)
            .bind(princess::tl_skill(&card.skill[0]))
            .bind(princess::tl_center_skill(&card.center_effect))
            .bind(card.vocal_min)
            .bind(card.dance_min)
            .bind(card.visual_min)
            .bind(card.vocal_max)
            .bind(card.dance_max)
            .bind(card.visual_max)
            .bind(card.vocal_min_awakened)
            .bind(card.dance_min_awakened)
            .bind(card.visual_min_awakened)
            .bind(card.vocal_max_awakened)
            .bind(card.dance_max_awakened)
            .bind(card.visual_max_awakened)
            .bind(card.resource_id)
            .execute(&self.pool).await?;
    
        Ok(())
    }

    async fn check_exists(&self) -> Result<bool, Error> {
        #[derive(sqlx::FromRow)]
        struct Tables { name: String }
        let check = sqlx::query_as::<_, Tables>("SELECT name FROM sqlite_master WHERE type='table' AND name='cards'")
            .fetch_all(&self.pool)
            .await?;

        Ok(check.len() == 0)
    }

    async fn handle_image(client: &reqwest::Client,
        file_path: String, url: String) -> Result<image::Handle, Error> {
            if Path::new(&file_path).exists() == true {
                Ok(image::Handle::from(&file_path))
            } else {
                let data = client.get(&url).send().await?.bytes().await?;

                tokio::fs::write(&file_path, data).await?;
                Ok(image::Handle::from_path(&file_path))
            }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    SqlxError,
    APIError,
    IoError
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
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

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        dbg!(error);

        Error::IoError
    }
}
