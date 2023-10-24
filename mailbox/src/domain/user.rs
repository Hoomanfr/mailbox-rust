use std::error::Error;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub id: String,
    pub user_name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(user_name: String, password: String) -> Result<User, Box<dyn Error>> {
        if user_name == String::from("") {
            return Err(Box::from("user name is empty"));
        }
        Ok(User {
            id: Uuid::new_v4().to_string(),
            user_name: user_name,
            password: password,
            created_at: Utc::now(),
        })
    }
}
