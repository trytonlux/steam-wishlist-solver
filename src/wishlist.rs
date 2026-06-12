use std::{
    fs::{self},
    path::Path,
};

use serde_json::{Map, Value};

pub type GameList = Vec<Game>;

#[derive(Debug, Clone)]
pub struct Game {
    pub appid: String,
    pub name: String,
    pub price: f32,
    pub discount: i64,
}

impl Game {
    fn from_value(value: &Map<String, Value>) -> Option<Self> {
        let appid = value
            .get("gameid")
            .unwrap()
            .as_array()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .unwrap()
            .replace("app/", "");

        let name = value.get("title").unwrap().as_str().unwrap().to_string();

        let price_value = value.get("price").unwrap();
        let price = match price_value.as_str() {
            Some(s) => s.replace("CDN$ ", "").parse::<f32>().unwrap(),
            None => return None,
        };

        let discount = value.get("discount").unwrap().as_i64().unwrap();

        Some(Game {
            appid,
            name,
            price,
            discount,
        })
    }
}

pub fn from_file(path: &Path) -> GameList {
    let file = fs::read_to_string(path).expect("unable to read file");
    let data: Value = serde_json::from_str(&file).expect("iunable to parse file");

    let data_games = data
        .get("data")
        .expect("Can't index data")
        .as_array()
        .expect("data not an array");

    let games: Vec<Game> = data_games
        .iter()
        .filter_map(|v| Game::from_value(v.as_object().unwrap()))
        .collect();

    games
}
