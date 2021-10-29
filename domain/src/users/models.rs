use repository::users::models::{InsertMyUsers, MyUsers};
use serde::Serialize;
use types::ID;

#[derive(Serialize, Debug, Clone)]
pub struct Users {
    pub id: ID,
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}

impl From<MyUsers> for Users {
    fn from(u: MyUsers) -> Self {
        Self {
            id: u.id,
            username: u.username,
            password: u.password,
            roles: u.roles,
        }
    }
}

impl From<Users> for InsertMyUsers {
    fn from(u: Users) -> Self {
        Self {
            username: u.username,
            password: u.password,
            roles: u.roles,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct RegistryUsers {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}
