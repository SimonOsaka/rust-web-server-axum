use crate::{
    database_to_domain_error, my_to_searched, search_to_domain_error, CreateAdventureError,
    DeleteAdventureError, NewJourneyData, Users,
};
use crate::{Adventures, AdventuresQuery, DomainError, GetAdventureError, PlayListQuery};

use anyhow::Result;

use repository::db::Repo;
use repository::{
    create_journey, delete_one_adventure, find_adventure_title_crypto, find_adventures_by_user_id,
    find_one_adventure, NewMyAdventuresJourney,
};
use search::adventures::{
    add_adventure, add_adventures, delete_adventure, search_by_play_list, search_latest, search_one,
};
use tracing::debug;
use vars::ID;

#[derive(Clone, Debug)]
pub struct AdventuresManagerImpl;

#[async_trait]
impl super::AdventuresManager for AdventuresManagerImpl {
    #[tracing::instrument(skip(self))]
    async fn find_adventures(
        &self,
        query: AdventuresQuery,
    ) -> Result<Vec<Adventures>, DomainError> {
        let search_results = search_latest(query.into()).await;
        let result = search_results
            .map_err(search_to_domain_error)?
            .into_iter()
            .map(Adventures::from)
            .collect();

        debug!("find_adventures result: {:?}", result);
        Ok(result)
    }

    #[tracing::instrument(skip(self))]
    async fn find_adventures_by_play_list(
        &self,
        query: PlayListQuery,
    ) -> Result<Vec<Adventures>, DomainError> {
        let search_results = search_by_play_list(query.into()).await;

        let result = search_results
            .map_err(search_to_domain_error)?
            .into_iter()
            .map(Adventures::from)
            .collect();

        debug!("find_adventures_by_play_list result: {:?}", result);
        Ok(result)
    }

    #[tracing::instrument(skip(self))]
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

    #[tracing::instrument(skip(self))]
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

    #[tracing::instrument(skip(self))]
    async fn sync_db_to_documents(&self, id: ID) -> Result<bool, DomainError> {
        let result = find_one_adventure(id, None).await;
        match result {
            Ok(opt_my) => match opt_my {
                Some(my) => Ok(add_adventure(my_to_searched(my))
                    .await
                    .map_err(search_to_domain_error)?),
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

    #[tracing::instrument(skip(self))]
    async fn add_journey(&self, data: NewJourneyData) -> Result<ID, CreateAdventureError> {
        let mut transaction = Repo::transaction().await.expect("");

        let my_adventures_optional =
            find_adventure_title_crypto(data.u.id, data.nj.crypto(), Some(&mut transaction))
                .await
                .map_err(database_to_domain_error)?;
        if my_adventures_optional.is_some() {
            return Err(CreateAdventureError::Exist);
        }

        let id = create_journey(NewMyAdventuresJourney::from(data), Some(&mut transaction))
            .await
            .map_err(database_to_domain_error)?;

        let result = find_one_adventure(id, Some(&mut transaction))
            .await
            .map_err(database_to_domain_error)?;

        transaction.commit().await.expect("");

        let status = match result {
            Some(ad) => add_adventures(vec![my_to_searched(ad)])
                .await
                .map_err(search_to_domain_error)?,
            None => return Err(CreateAdventureError::AdventureNotFound { adventure_id: id }),
        };

        if status {
            Ok(id)
        } else {
            Err(CreateAdventureError::AddDocuments)
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete_adventure(&self, id: ID, user_id: ID) -> Result<bool, DeleteAdventureError> {
        let mut transaction = Repo::transaction().await.expect("");

        let result = find_one_adventure(id, Some(&mut transaction))
            .await
            .map_err(|e| DeleteAdventureError::DomainError(database_to_domain_error(e)))?;
        if result.is_none() {
            debug!("adventure {} is not exist", id);
            return Ok(true);
        } else if result.unwrap().user_id != user_id {
            debug!("adventure {} 's owner is not the user {}", id, user_id);
            return Err(DeleteAdventureError::NotOwner);
        }

        let is_db_del = delete_one_adventure(id, Some(&mut transaction))
            .await
            .map_err(|e| DeleteAdventureError::DomainError(database_to_domain_error(e)))?;

        transaction.commit().await.expect("commit error");

        if is_db_del {
            let status = delete_adventure(id)
                .await
                .map_err(|e| DeleteAdventureError::DomainError(search_to_domain_error(e)))?;

            if status {
                return Ok(true);
            } else {
                return Err(DeleteAdventureError::DelDocuments);
            }
        }

        Ok(true)
    }

    #[tracing::instrument(skip(self))]
    async fn find_by_user_id(&self, user_id: ID) -> Result<Vec<(Adventures, Users)>, DomainError> {
        let result = find_adventures_by_user_id(user_id, None)
            .await
            .map_err(database_to_domain_error)?;

        let t = result
            .into_iter()
            .map(|db_my| {
                let (ad, u) = db_my;
                (Adventures::from(ad), Users::from(u))
            })
            .collect();

        Ok(t)
    }
}
