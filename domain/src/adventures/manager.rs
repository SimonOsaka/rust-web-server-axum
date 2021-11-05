use anyhow::Result;
use types::ID;

use crate::{
    Adventures, AdventuresQuery, CreateAdventureError, DomainError, GetAdventureError,
    NewJourneyData, PlayListQuery,
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
}
