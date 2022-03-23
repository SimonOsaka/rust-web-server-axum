use anyhow::Result;
use vars::ID;

use crate::{
    Adventures, AdventuresQuery, CreateAdventureError, DeleteAdventureError, DomainError,
    GetAdventureError, NewJourneyData, PlayListQuery, Users,
};
#[async_trait]
pub trait AdventuresManager {
    /// adventure list
    async fn find_adventures(&self, query: AdventuresQuery)
        -> Result<Vec<Adventures>, DomainError>;

    /// play_list
    async fn find_adventures_by_play_list(
        &self,
        query: PlayListQuery,
    ) -> Result<Vec<Adventures>, DomainError>;

    /// one adventure
    async fn get_adventure_by_id(&self, id: ID) -> Result<Option<Adventures>, GetAdventureError>;

    async fn get_adventure(&self, id: ID) -> Result<Adventures, GetAdventureError>;

    async fn sync_db_to_documents(&self, id: ID) -> Result<bool, DomainError>;

    async fn add_journey(&self, data: NewJourneyData) -> Result<ID, CreateAdventureError>;

    async fn delete_adventure(&self, id: ID, user_id: ID) -> Result<bool, DeleteAdventureError>;

    async fn find_by_user_id(&self, user_id: ID) -> Result<Vec<(Adventures, Users)>, DomainError>;
}
