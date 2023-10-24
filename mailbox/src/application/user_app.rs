use std::error::Error;

use serde::{Serialize, Deserialize};

use crate::domain::user::User;
use crate::infrastructure::db::DbFactory;

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub user_name: String,
}

pub struct UserApp {
    pub db_factory: DbFactory,
}

impl UserApp {
    pub fn new(db_factory: DbFactory) -> Self {
        UserApp { db_factory }
    }

    pub async fn create(&self, user_req: UserRequest) -> Result<String, Box<dyn Error>> {
        let user = User::new(user_req.user_name, user_req.password)?;
        self.db_factory.user_db().create(&user).await?;
        return Ok(user.id);
    }

    pub async fn find_by_id(&self, id: String) -> Result<Option<UserResponse>, Box<dyn Error>> {
        if let Some(user) = self.db_factory.user_db().find_by_id(id).await? {
            return Ok(Some(UserResponse {
                id: user.id,
                user_name: user.user_name,
            }));
        }
        Ok(None)
    }
}
