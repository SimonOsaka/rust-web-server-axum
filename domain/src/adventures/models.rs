use repository::models::PlayListWhere;
use serde::Serialize;
use types::{DateTime, ID, U8I16};

#[derive(Clone, Debug)]
pub struct AdventuresQuery {
    pub item_id: u8,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub province_key: Option<String>,
}

impl From<repository::models::AdventuresWhere> for AdventuresQuery {
    fn from(w: repository::models::AdventuresWhere) -> Self {
        Self {
            item_id: w.item_id,
            limit: w.limit,
            offset: w.offset,
            province_key: w.province_key,
        }
    }
}

impl Into<repository::models::AdventuresWhere> for AdventuresQuery {
    fn into(self) -> repository::models::AdventuresWhere {
        repository::models::AdventuresWhere {
            item_id: (self.item_id),
            limit: (self.limit),
            offset: (self.offset),
            province_key: (self.province_key),
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Adventures {
    pub id: ID,
    pub title: String,
    pub image_url: String,
    pub created_at: DateTime,
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

#[derive(Clone, Debug, PartialEq)]
pub struct AdventuresUpdate {
    pub id: u64,
    pub title: String,
    pub image_url: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct AdventureContent {
    pub title: String,
    pub image_url: String,
}

#[derive(Clone, Debug)]
pub struct PlayListQuery {
    pub play_list: String,
}

impl Into<PlayListWhere> for PlayListQuery {
    fn into(self) -> PlayListWhere {
        PlayListWhere {
            play_list: (self.play_list),
        }
    }
}
