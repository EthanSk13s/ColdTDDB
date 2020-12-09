#[macro_use]
extern crate lazy_static;

mod gui;
mod db;
mod princess;
mod components;
mod app;
use std::path::Path;
use iced::{Application, Settings, window};
use rusqlite::Connection;

fn main() -> iced::Result {
    // Hacky way to make a DB in runtime, but sqlx does not support it yet?
    if Path::new("ayaya.db").exists() == false {
        let _conn = Connection::open("ayaya.db");
    };

    let setting = Settings {
        default_text_size: 20,
        window: window::Settings {
            size: (1280, 720),
            ..window::Settings::default()
        },
        flags: (),
        default_font: Some(include_bytes!("../fonts/rounded-mgenplus-2cp-regular.ttf")),
        antialiasing: false
    
    };

    return app::App::run(setting);
}