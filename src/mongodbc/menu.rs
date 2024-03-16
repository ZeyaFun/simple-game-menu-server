use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    pub _id: i64,
    pub content: String,
    pub link: String,
}

impl Menu {
    pub fn new(_id: i64, content: String, link: String) -> Menu {
        Menu { _id, content, link }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotFound {
    pub content: String,
}
