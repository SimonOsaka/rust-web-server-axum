use crate::{
    AdventuresManager, CreateAdventureError, DomainError, NewJourney, NewJourneyData, UsersManager,
};
use repository::users::models::{MyUsers, NewMyUsers};
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

impl Users {
    pub async fn change_password(
        &self,
        new_password: String,
        manager: &impl UsersManager,
    ) -> Result<bool, DomainError> {
        Ok(manager
            .change_password(self.username.to_string(), new_password)
            .await?)
    }

    pub async fn change_username(
        &self,
        new_username: String,
        manager: &impl UsersManager,
    ) -> Result<bool, DomainError> {
        Ok(manager
            .change_username(self.username.to_string(), new_username)
            .await?)
    }

    pub async fn add_journey(
        &self,
        new_journey: NewJourney,
        manager: &impl AdventuresManager,
    ) -> Result<ID, CreateAdventureError> {
        Ok(manager
            .add_journey(NewJourneyData {
                nj: new_journey,
                u: self.to_owned(),
            })
            .await?)
    }
}

impl From<Users> for NewMyUsers {
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
