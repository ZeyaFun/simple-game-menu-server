use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub _id: String,
    pub image: String,
    pub name: String,
    pub description: String,
    pub download_link: String,
    pub game_path: String,
}

impl Game {
    pub fn new(
        _id: String,
        image: String,
        name: String,
        description: String,
        download_link: String,
        game_path: String,
    ) -> Game {
        Game {
            _id,
            image,
            name,
            description,
            download_link,
            game_path,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub _id: ObjectId,
    pub image_path: String,
    pub name: String,
    pub description: String,
    pub download_link: String,
    pub game_path: String,
}
