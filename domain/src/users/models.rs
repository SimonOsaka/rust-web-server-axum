use crate::{
    adventures::Adventures, adventures::AdventuresManager,
    adventures::CreateAdventureError, adventures::DeleteAdventureError,
    adventures::NewJourney, adventures::NewJourneyData, favorites::AddFavorite,
    favorites::DelFavorite, favorites::Favorite, favorites::FavoriteError,
    favorites::FavoritesManager, DomainError, UsersManager,
};
use repository::users::models::{MyUsers, NewMyUsers};
use serde::Serialize;
use vars::ID;

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
        manager
            .change_password(self.username.to_string(), new_password)
            .await
    }

    pub async fn change_username(
        &self,
        new_username: String,
        manager: &impl UsersManager,
    ) -> Result<bool, DomainError> {
        manager
            .change_username(self.username.to_string(), new_username)
            .await
    }

    pub async fn add_journey(
        &self,
        new_journey: NewJourney,
        manager: &impl AdventuresManager,
    ) -> Result<ID, CreateAdventureError> {
        manager
            .add_journey(NewJourneyData {
                nj: new_journey,
                u: self.to_owned(),
            })
            .await
    }

    pub async fn favorite(
        &self,
        adventure_id: ID,
        manager: &impl FavoritesManager,
    ) -> Result<Favorite, FavoriteError> {
        manager
            .add_favorite(AddFavorite {
                user_id: self.id,
                adventure_id,
            })
            .await
    }

    pub async fn unfavorite(
        &self,
        adventure_id: ID,
        manager: &impl FavoritesManager,
    ) -> Result<bool, FavoriteError> {
        manager
            .del_favorite(DelFavorite {
                user_id: self.id,
                adventure_id,
            })
            .await
    }

    pub async fn delete_adventure(
        &self,
        adventure_id: ID,
        manager: &impl AdventuresManager,
    ) -> Result<bool, DeleteAdventureError> {
        manager.delete_adventure(adventure_id, self.id).await
    }

    pub async fn find_adventures(
        &self,
        manager: &impl AdventuresManager,
    ) -> Result<Vec<(Adventures, Users)>, DomainError> {
        manager.find_by_user_id(self.id).await
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
