use crate::{Adventures, AdventuresQuery, DatabaseError, GetAdventureError, PlayListQuery};
use anyhow::Error as OpaqueError;
use anyhow::Result;

use log::debug;
use meilisearch_sdk::errors::Error;
use meilisearch_sdk::search::SearchResult;
use repository::models::MyAdventures;
use repository::SqlxError;
use search::operation::Condition;
use search::operation::Page;
use search::operation::Sort;
use search::operation::SortDirect;
use search::operation::SortProperty;
use types::ID;
pub fn to_db_error(e: SqlxError) -> DatabaseError {
    DatabaseError::from(OpaqueError::from(e))
}

#[derive(Clone, Debug)]
pub struct Manager;

#[async_trait]
impl crate::manager::Manager for Manager {
    async fn find_adventures(
        &self,
        query: AdventuresQuery,
    ) -> Result<Vec<Adventures>, DatabaseError> {
        let mut filter = vec!["is_deleted = 0".to_string()];
        if query.item_id != 0 {
            filter.push(format!("item_type = {}", query.item_id));
        }
        if let Some(pv) = query.province_key {
            filter.push(format!("journey_destiny = {:?}", pv))
        }

        debug!("filter: {:?}", filter);

        let mut condition = Condition::new();
        condition.filter = Some(filter.join(" AND "));
        condition.sort = Some(Sort {
            property: SortProperty::ID,
            direct: SortDirect::DESC,
        });
        condition.page = Some(Page::from(
            query.limit.unwrap_or(10),
            query.offset.unwrap_or(0),
        ));

        let search_results =
            search::operation::search_documents_with_filter::<MyAdventures>(condition).await;

        let result: Vec<Adventures> = search_results
            .unwrap()
            .into_iter()
            .map(|sr| {
                let my = sr.result;
                Adventures::from(my)
            })
            .collect();
        Ok(result)
    }

    async fn find_adventures_by_play_list(
        &self,
        query: PlayListQuery,
    ) -> Result<Vec<Adventures>, DatabaseError> {
        let mut condition = Condition::new();
        condition.filter = Some(format!(
            "play_list = {} AND is_deleted = 0",
            query.play_list
        ));
        condition.page = Some(Page::of(1));
        let search_results: Result<Vec<SearchResult<MyAdventures>>, Error> =
            search::operation::search_documents_with_filter::<MyAdventures>(condition).await;

        let result: Vec<Adventures> = search_results
            .unwrap()
            .into_iter()
            .map(|sr| {
                let my = sr.result;
                Adventures::from(my)
            })
            .collect();

        Ok(result)
    }

    async fn get_adventure_by_id(&self, id: ID) -> Result<Option<Adventures>, GetAdventureError> {
        let mut condition = Condition::new();
        condition.filter = Some(format!("id = {} AND is_deleted = 0", id));
        condition.page = Some(Page::one());
        let search_results: Result<Vec<SearchResult<MyAdventures>>, Error> =
            search::operation::search_documents_with_filter::<MyAdventures>(condition).await;

        let result;
        if search_results.as_ref().unwrap().len() == 1 {
            let mut sr_my: Vec<Adventures> = search_results
                .unwrap()
                .into_iter()
                .map(|sr| {
                    let my = sr.result;
                    Adventures::from(my)
                })
                .collect();

            result = sr_my.pop();

            debug!("get result after search: {:?}", result);
        } else {
            let my = repository::adventures::find_one(id)
                .await
                .map_err(|e| match e {
                    e @ repository::SqlxError::RowNotFound => GetAdventureError::NotFound {
                        adventure_id: id,
                        source: to_db_error(e),
                    },
                    e => to_db_error(e).into(),
                })
                .unwrap();

            if let Some(ref m) = my {
                search::operation::add_documents(vec![m.clone()]).await;
            }

            result = match my {
                Some(ad) => Some(Adventures::from(ad)),
                _ => None,
            };

            debug!("get result after db: {:?}", result);
        }
        Ok(result)
    }
}
