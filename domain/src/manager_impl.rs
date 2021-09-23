use crate::{Adventures, AdventuresQuery, DatabaseError, GetAdventureError, PlayListQuery};
use anyhow::Error as OpaqueError;
use anyhow::Result;
use repository::SqlxError;
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
        let my_list_result = repository::adventures::find_latest(query.into()).await;
        let result: Vec<Adventures> = my_list_result
            .map_err(to_db_error)
            .unwrap()
            .into_iter()
            .map(|m| Adventures {
                id: m.id,
                title: m.title,
                image_url: m.image_url,
                created_at: m.created_at,
                item_type: m.item_type,
                link: m.link,
                source: m.source,
                journey_destiny: m.journey_destiny,
                script_content: m.script_content,
                play_list: m.play_list,
                address: m.address,
                shop_name: m.shop_name,
                province: m.province,
                city: m.city,
                district: m.district,
            })
            .collect();
        Ok(result)
    }

    async fn find_adventures_by_play_list(
        &self,
        query: PlayListQuery,
    ) -> Result<Vec<Adventures>, DatabaseError> {
        let my_list_result = repository::adventures::find_by_play_list(query.into()).await;
        let result: Vec<Adventures> = my_list_result
            .map_err(to_db_error)
            .unwrap()
            .into_iter()
            .map(|m| Adventures {
                id: m.id,
                title: m.title,
                image_url: m.image_url,
                created_at: m.created_at,
                item_type: m.item_type,
                link: m.link,
                source: m.source,
                journey_destiny: m.journey_destiny,
                script_content: m.script_content,
                play_list: m.play_list,
                address: m.address,
                shop_name: m.shop_name,
                province: m.province,
                city: m.city,
                district: m.district,
            })
            .collect();
        Ok(result)
    }

    async fn get_adventure_by_id(&self, id: ID) -> Result<Option<Adventures>, GetAdventureError> {
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

        let result = match my {
            Some(ad) => Some(Adventures {
                id: ad.id,
                title: ad.title,
                image_url: ad.image_url,
                created_at: ad.created_at,
                item_type: ad.item_type,
                link: ad.link,
                source: ad.source,
                journey_destiny: ad.journey_destiny,
                script_content: ad.script_content,
                play_list: ad.play_list,
                address: ad.address,
                shop_name: ad.shop_name,
                province: ad.province,
                city: ad.city,
                district: ad.district,
            }),
            _ => None,
        };

        Ok(result)
    }
}
