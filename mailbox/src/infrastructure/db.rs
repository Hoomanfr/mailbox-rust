use super::{
    mailbox_db::{MailboxDB, MailboxDb},
    user_db::{UserDB, UserDb},
};
use rustlib::{config::vault::ConfigManager, database::postgresql::PgDb};
use std::sync::Arc;

type MailboxData = Box<dyn MailboxDB + Send + Sync>;
type UserData = Box<dyn UserDB + Send + Sync>;

pub type DbFactory = Arc<dyn DBFactory + Send + Sync>;
pub trait DBFactory {
    fn mailbox_db(&self) -> &MailboxData;
    fn user_db(&self) -> &UserData;
}

pub struct DbFactoryImpl {
    mailbox_data: MailboxData,
    user_data: UserData,
}

impl DbFactoryImpl {
    pub async fn new(cfg: &ConfigManager) -> DbFactory {
        let pgdb = PgDb::new(cfg).await.unwrap();
        Arc::new(DbFactoryImpl {
            mailbox_data: Box::new(MailboxDb::new(pgdb.clone())),
            user_data: Box::new(UserDb::new(pgdb.clone())),
        })
    }
}

impl DBFactory for DbFactoryImpl {
    fn mailbox_db(&self) -> &MailboxData {
        return &self.mailbox_data;
    }

    fn user_db(&self) -> &UserData {
        return &self.user_data;
    }
}
