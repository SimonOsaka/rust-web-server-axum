use sqlx::Error;
use types::{DateTime, ID, U8I16};

use crate::insert;

#[derive(sqlx::FromRow, Debug)]
pub struct MyUsers {
    pub id: ID,
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
    pub is_deleted: U8I16,
    pub created_at: DateTime,
}

pub struct InsertMyUsers {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}

impl InsertMyUsers {
    pub async fn add_user(&self) -> Result<ID, Error> {
        let Self {
            username,
            password,
            roles,
        } = self;

        let inserted_my_user_id = insert(InsertMyUsers {
            username: username.to_string(),
            password: password.to_string(),
            roles: roles.to_vec(),
        })
        .await?;

        Ok(inserted_my_user_id)
    }
}
