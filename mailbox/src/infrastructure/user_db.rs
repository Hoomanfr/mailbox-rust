use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rustlib::database::postgresql::PgDb;
use std::error::Error;
use sqlx::{postgres::PgRow, Row};
use crate::domain::user::*;

#[async_trait]
pub trait UserDB {
    async fn create(&self, user: &User) -> Result<(), Box<dyn Error>>;
    async fn find_by_id(&self, id: String) -> Result<Option<User>, Box<dyn Error>>;
}

pub struct UserDb {
    pool: PgDb,
}

impl UserDb {
    pub fn new(pool: PgDb) -> Self {
        UserDb { pool: pool }
    }
}

#[async_trait]
impl UserDB for UserDb {
    async fn create(&self, user: &User) -> Result<(), Box<dyn Error>> {
        sqlx::query("INSERT INTO users (id, user_name, password, created_at) VALUES ($1, $2, $3, $4);")
            .bind(&user.id)
            .bind(&user.user_name)
            .bind(&user.password)
            .bind(&user.created_at)
            .execute(&self.pool.pool)
            .await?;
        Ok(())
    }

    async fn find_by_id(&self, id: String) -> Result<Option<User>, Box<dyn Error>> {
        let mut user: Option<User> = None;
        sqlx::query("SELECT id, user_name, password, created_at FROM users WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| {
                user = Some(User {
                    id: row.get::<String, _>(0),
                    user_name: row.get::<String, _>(1),
                    password: row.get::<String, _>(2),
                    created_at: row.get::<DateTime<Utc>, _>(3),
                })
            })
            .fetch_optional(&self.pool.pool)
            .await?;
        return Ok(user);
    }
}
