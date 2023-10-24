use std::error::Error;

use chrono::{DateTime, Utc};
use regex::Regex;
use uuid::Uuid;


pub struct Mailbox{
    pub id: String,
    pub user_id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl Mailbox{
    pub fn new(user_id: String, email: String) -> Result<Mailbox, Box<dyn Error>>{
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        if !email_regex.is_match(&email){
            return Err(Box::from("invalid email"));
        }
        Ok(Mailbox{
            id: Uuid::new_v4().to_string(),
            user_id,
            email,
            created_at: Utc::now(),
        })
    }
}
