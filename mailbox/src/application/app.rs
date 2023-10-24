use std::sync::Arc;

use crate::infrastructure::db::DbFactory;

use super::{mailbox_app::MailboxApp, user_app::UserApp};

pub type AppFactory = Arc<dyn ApplicationFactory + Send + Sync>;

pub trait ApplicationFactory{
    fn mailbox_app(&self) -> &MailboxApp;
    fn user_app(&self) -> &UserApp;
}

pub struct ApplicationFactoryImpl{
    mailbox_app: MailboxApp,
    user_app: UserApp,
}

impl ApplicationFactoryImpl{
    pub fn new(db_factory: DbFactory) -> AppFactory{
        return Arc::new(ApplicationFactoryImpl{
            mailbox_app: MailboxApp::new(db_factory.clone()),
            user_app: UserApp::new(db_factory.clone()),
        });
    }
}

impl ApplicationFactory for ApplicationFactoryImpl{
    fn mailbox_app(&self) -> &MailboxApp{
        return &self.mailbox_app;
    }

    fn user_app(&self) -> &UserApp{
        return &self.user_app;
    }
}