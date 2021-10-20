use anyhow::Result;
use types::ID;

use crate::{Adventures, AdventuresQuery, DomainError, GetAdventureError, PlayListQuery};
#[async_trait]
pub trait Manager {
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
}
