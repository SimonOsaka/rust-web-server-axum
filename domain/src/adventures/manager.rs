use std::future::Future;

use anyhow::Result;
use vars::ID;

use crate::{
    adventures::Adventures, adventures::AdventuresQuery,
    adventures::CreateAdventureError, adventures::DeleteAdventureError,
    adventures::GetAdventureError, adventures::NewJourneyData,
    adventures::PlayListQuery, DomainError, Users,
};

pub trait AdventuresManager {
    /// adventure list
    fn find_adventures(
        &self,
        query: AdventuresQuery,
    ) -> impl Future<Output = Result<Vec<Adventures>, DomainError>>;

    /// play_list
    fn find_adventures_by_play_list(
        &self,
        query: PlayListQuery,
    ) -> impl Future<Output = Result<Vec<Adventures>, DomainError>>;

    /// one adventure
    fn get_adventure_by_id(
        &self,
        id: ID,
    ) -> impl Future<Output = Result<Option<Adventures>, GetAdventureError>>;

    fn get_adventure(
        &self,
        id: ID,
    ) -> impl Future<Output = Result<Adventures, GetAdventureError>> + Send;

    fn sync_db_to_documents(
        &self,
        id: ID,
    ) -> impl Future<Output = Result<bool, DomainError>>;

    fn add_journey(
        &self,
        data: NewJourneyData,
    ) -> impl Future<Output = Result<ID, CreateAdventureError>>;

    fn delete_adventure(
        &self,
        id: ID,
        user_id: ID,
    ) -> impl Future<Output = Result<bool, DeleteAdventureError>>;

    fn find_by_user_id(
        &self,
        user_id: ID,
    ) -> impl Future<Output = Result<Vec<(Adventures, Users)>, DomainError>>;
}
