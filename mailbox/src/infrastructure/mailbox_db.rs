use crate::domain::mailbox::*;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rustlib::database::postgresql::PgDb;
use sqlx::{postgres::PgRow, Row};
use std::error::Error;

#[async_trait]
pub trait MailboxDB {
    async fn create(&self, mailbox: &Mailbox) -> Result<(), Box<dyn Error>>;
    async fn find_by_user_id(&self, user_id: String) -> Result<Option<Vec<Mailbox>>, Box<dyn Error>>;
}

pub struct MailboxDb {
    pool: PgDb,
}

impl MailboxDb {
    pub fn new(pool: PgDb) -> Self {
        MailboxDb { pool: pool }
    }
}

#[async_trait]
impl MailboxDB for MailboxDb {
    async fn create(&self, mailbox: &Mailbox) -> Result<(), Box<dyn Error>> {
        sqlx::query("INSERT INTO mailboxes (id, user_id, email, created_at) VALUES ($1, $2, $3, $4)")
            .bind(&mailbox.id)
            .bind(&mailbox.user_id)
            .bind(&mailbox.email)
            .bind(&mailbox.created_at)
            .execute(&self.pool.pool)
            .await?;
        return Ok(())
    }
    async fn find_by_user_id(&self, user_id: String) -> Result<Option<Vec<Mailbox>>, Box<dyn Error>> {
        let mut mailboxes = Vec::new();
        sqlx::query("SELECT id, user_id, email, created_at FROM mailboxes WHERE user_id = $1")
            .bind(user_id)
            .map(|row: PgRow| {
                let mailbox = Mailbox {
                    id: row.get::<String, _>(0),
                    user_id: row.get::<String, _>(1),
                    email: row.get::<String, _>(2),
                    created_at: row.get::<DateTime<Utc>, _>(3),
                };
                mailboxes.push(mailbox);
            })
            .fetch_all(&self.pool.pool)
            .await?;
        if mailboxes.len() > 0 {
            return Ok(Some(mailboxes));
        }
        Ok(None)   
    }
}
