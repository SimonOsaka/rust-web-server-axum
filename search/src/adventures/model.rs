use serde::{Deserialize, Serialize};
use vars::{DateTime, ID};

#[derive(Clone, Debug)]
pub struct AdventuresFilter {
    pub item_id: u8,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub province_key: Option<String>,
}

#[derive(Clone, Debug)]
pub struct PlayListFilter {
    pub play_list: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SearchedAdventures {
    pub id: ID,
    pub title: String,
    pub image_url: String,
    pub created_at: DateTime,
    pub is_deleted: i16,
    pub item_type: i16,
    pub link: String,
    pub source: i16,
    pub journey_destiny: String,
    pub script_content: String,
    pub play_list: String,
    pub address: String,
    pub shop_name: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub user_id: ID,
    pub fav_count: i64,
}
