use crate::{
    database_to_domain_error, search_to_domain_error, CreateAdventureError, NewJourneyData,
};
use crate::{Adventures, AdventuresQuery, DomainError, GetAdventureError, PlayListQuery};

use anyhow::Result;

use log::debug;

use meilisearch_sdk::progress::UpdateStatus;
use repository::{create_journey, find_one, NewMyAdventuresJourney};
use search::adventures::{search_by_play_list, search_latest, search_one};
use search::meilisearch::operation::add_documents;
use types::ID;

#[derive(Clone, Debug)]
pub struct AdventuresManagerImpl;

#[async_trait]
impl super::AdventuresManager for AdventuresManagerImpl {
    async fn find_adventures(
        &self,
        query: AdventuresQuery,
    ) -> Result<Vec<Adventures>, DomainError> {
        let search_results = search_latest(query.into()).await;
        let result = search_results
            .map_err(search_to_domain_error)?
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
        let search_results = search_by_play_list(query.into()).await;

        let result = search_results
            .map_err(search_to_domain_error)?
            .into_iter()
            .map(|my| Adventures::from(my))
            .collect();

        debug!("find_adventures_by_play_list result: {:?}", result);
        Ok(result)
    }

    async fn get_adventure_by_id(&self, id: ID) -> Result<Option<Adventures>, GetAdventureError> {
        let search_results = search_one(id).await.map_err(search_to_domain_error);

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
        let search_results = search_one(id).await.map_err(search_to_domain_error);

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
        let result = search_one(id).await;
        match result {
            Ok(opt_my) => match opt_my {
                Some(my) => {
                    add_documents(vec![my])
                        .await
                        .map_err(search_to_domain_error)
                        .unwrap();
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

    async fn add_journey(&self, data: NewJourneyData) -> Result<ID, CreateAdventureError> {
        let id = create_journey(NewMyAdventuresJourney::from(data))
            .await
            .map_err(database_to_domain_error)?;

        let result = find_one(id).await.map_err(database_to_domain_error)?;

        let progress = match result {
            Some(ad) => add_documents(vec![ad])
                .await
                .map_err(search_to_domain_error)?,
            None => return Err(CreateAdventureError::AdventureNotFound { adventure_id: id }),
        };

        match progress.wait_for_pending_update(None, None).await {
            Some(o) => match o {
                Ok(s) => match s {
                    UpdateStatus::Processed { .. } => return Ok(id),
                    _ => return Err(CreateAdventureError::AddDocuments),
                },
                Err(_) => return Err(CreateAdventureError::AddDocuments),
            },
            None => return Err(CreateAdventureError::AddDocuments),
        }
    }
}
