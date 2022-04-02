use serde::{Deserialize, Serialize};
use vars::{DateTime, ID, U8I16};

use crate::MyUsers;

#[derive(sqlx::FromRow, sqlx::Type, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[sqlx(type_name = "RECORD")]
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
    pub user_id: ID,
    pub fav_count: i64,
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

#[derive(Debug)]
pub struct NewMyAdventuresJourney {
    pub title: String,
    pub title_crypto: String,
    pub image_url: String,
    pub item_type: U8I16,
    pub link: String,
    pub source: U8I16,
    pub journey_destiny: String,
    pub user_id: ID,
}

pub struct FindAllWhere {
    pub user_id: ID,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AdventureUser {
    pub my_adventures: MyAdventures,
    pub my_users: MyUsers,
}

#[derive(Debug)]
pub enum FavCountKind {
    Fav,
    UnFav,
}
