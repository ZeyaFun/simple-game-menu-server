use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: i64,
    pub user_name: String,
    pub password: String,
}

impl User {
    pub fn new(_id: i64, user_name: String, password: String) -> User {
        User {
            _id,
            user_name,
            password,
        }
    }
}
