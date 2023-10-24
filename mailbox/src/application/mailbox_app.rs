use std::error::Error;

use serde::{Serialize, Deserialize};

use crate::{infrastructure::{db::DbFactory}, domain::mailbox::Mailbox};

#[derive(Serialize, Deserialize)]
pub struct MailboxRequest{
    pub user_id: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct MailboxResponse{
    pub id: String,
    pub user_id: String,
    pub email: String,
}

pub struct MailboxApp{
    pub db_factory: DbFactory,
}

impl MailboxApp{
    pub fn new(db_factory: DbFactory) -> Self{
        MailboxApp { db_factory }
    }

    pub async fn create(&self, mailbox_req: MailboxRequest) -> Result<String, Box<dyn Error>> {
        let mailbox = Mailbox::new(mailbox_req.user_id, mailbox_req.email)?;
        self.db_factory.mailbox_db().create(&mailbox).await?;
        return Ok(mailbox.id);
    }

    pub async fn find_by_user_id(&self, user_id: String) -> Result<Option<Vec<MailboxResponse>>, Box<dyn Error>> {
        if let Some(mailboxes) = self.db_factory.mailbox_db().find_by_user_id(user_id).await? {
            let mailboxes_resp = mailboxes.iter().map(|m| MailboxResponse {
                id: m.id.clone(),
                user_id: m.user_id.clone(),
                email: m.email.clone(),
            }).collect();
            return Ok(Some(mailboxes_resp));
        }
        Ok(None)
    }
}
