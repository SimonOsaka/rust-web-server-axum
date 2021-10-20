use crate::{Adventures, AdventuresQuery, DomainError, GetAdventureError, PlayListQuery};
use anyhow::Error as OpaqueError;
use anyhow::Result;

use log::debug;
use meilisearch_sdk::errors::Error;
use search::adventures::{find_by_play_list, find_latest, find_one};
use search::meilisearch::operation::add_documents;
use types::ID;
pub fn to_domain_error(e: Error) -> DomainError {
    DomainError::from(OpaqueError::from(e))
}

#[derive(Clone, Debug)]
pub struct Manager;

#[async_trait]
impl crate::manager::Manager for Manager {
    async fn find_adventures(
        &self,
        query: AdventuresQuery,
    ) -> Result<Vec<Adventures>, DomainError> {
        let search_results = find_latest(query.into()).await;
        let result = search_results
            .map_err(to_domain_error)?
            .into_iter()
            .map(|my| Adventures::from(my))
            .collect();

        debug!("find_adventures result: {:?}", result);
        Ok(result)
    }

    async fn find_adventures_by_play_list(
        &self,
        query: PlayListQuery,
    ) -> Result<Vec<Adventures>, DomainError> {
        let search_results = find_by_play_list(query.into()).await;

        let result = search_results
            .map_err(to_domain_error)?
            .into_iter()
            .map(|my| Adventures::from(my))
            .collect();

        debug!("find_adventures_by_play_list result: {:?}", result);
        Ok(result)
    }

    async fn get_adventure_by_id(&self, id: ID) -> Result<Option<Adventures>, GetAdventureError> {
        let search_results = find_one(id).await.map_err(to_domain_error);

        match search_results? {
            Some(my) => {
                let result = Some(Adventures::from(my));
                debug!("get_adventure_by_id result: {:?}", result);
                Ok(result)
            }
            None => Err(GetAdventureError::NotFound { adventure_id: id }),
        }
    }

    async fn get_adventure(&self, id: ID) -> Result<Adventures, GetAdventureError> {
        let search_results = find_one(id).await.map_err(to_domain_error);

        match search_results? {
            Some(my) => {
                let result = Adventures::from(my);
                debug!("get_adventure result: {:?}", result);
                Ok(result)
            }
            None => Err(GetAdventureError::NotFound { adventure_id: id }),
        }
    }

    async fn sync_db_to_documents(&self, id: ID) -> Result<bool, DomainError> {
        let result = find_one(id).await;
        match result {
            Ok(opt_my) => match opt_my {
                Some(my) => {
                    add_documents(vec![my]).await;
                    Ok(true)
                }
                None => {
                    println!("NONE, not exist");
                    Ok(false)
                }
            },
            _ => {
                println!("RESULT, ERR");
                Ok(false)
            }
        }
    }
}
