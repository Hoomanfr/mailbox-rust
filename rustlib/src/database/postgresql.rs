use sqlx::{postgres::{PgPool, PgPoolOptions}, Transaction, Postgres};
use std::{time::Duration, error::Error};
use crate::config::vault::ConfigManager;

#[derive(Clone)]
pub struct PgDb{
    pub pool: PgPool
}

impl PgDb{
    pub async fn new(cfg: &ConfigManager) -> Result<PgDb, Box<dyn Error>>{
        let db_url = cfg.get_value("DATABASE_URL".to_string()).await?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_url.as_str())
            .await?;
        Ok(PgDb{
            pool
        })
    }
    
    pub async fn with_connection(&mut self, f: impl FnOnce(&mut PgPool) -> Option<Box<dyn Error>>) -> Option<Box<dyn Error>>{
        return f(&mut self.pool);
    }

    pub async fn with_transaction(&mut self, f: impl FnOnce(&mut Transaction<'_, Postgres>) -> Option<Box<dyn Error>>) -> Option<Box<dyn Error>>{
        let mut tx = self.pool.begin().await.unwrap();
        let result = f(&mut tx);
        match result{
            Some(_) => {
                tx.rollback().await.unwrap();
            },
            None => {
                tx.commit().await.unwrap();
            }
        }
        return result;
    }
}