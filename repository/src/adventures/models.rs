use meilisearch_sdk::document::Document;
use serde::{Deserialize, Serialize};

use types::{DateTime, ID, U8I16};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EntityId {
    pub id: ID,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct MyAdventures {
    pub id: ID,
    pub title: String,
    pub image_url: String,
    pub created_at: DateTime,
    pub is_deleted: U8I16,
    pub item_type: U8I16,
    pub link: String,
    pub source: U8I16,
    pub journey_destiny: String,
    pub script_content: String,
    pub play_list: String,
    pub address: String,
    pub shop_name: String,
    pub province: String,
    pub city: String,
    pub district: String,
}

impl Document for MyAdventures {
    type UIDType = ID;
    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}

#[derive(Clone, Debug)]
pub struct AdventuresWhere {
    pub item_id: u8,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub province_key: Option<String>,
}

#[derive(Clone, Debug)]
pub struct PlayListWhere {
    pub play_list: String,
}
