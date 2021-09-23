use anyhow::Result;
use types::ID;

use crate::{Adventures, AdventuresQuery, DatabaseError, GetAdventureError, PlayListQuery};
#[async_trait]
pub trait Manager {
    // adventures
    async fn find_adventures(
        &self,
        query: AdventuresQuery,
    ) -> Result<Vec<Adventures>, DatabaseError>;

    async fn find_adventures_by_play_list(
        &self,
        query: PlayListQuery,
    ) -> Result<Vec<Adventures>, DatabaseError>;

    async fn get_adventure_by_id(&self, id: ID) -> Result<Option<Adventures>, GetAdventureError>;
}
