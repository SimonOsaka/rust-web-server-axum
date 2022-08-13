use crate::{myusers_fields_sqlquery, MyUsers, MyUsersFields, MYUSERS_MULTI_FIELDS};
use macros::FromModel;
use serde::{Deserialize, Serialize};
use vars::{DateTime, ID};

#[derive(
    FromModel, sqlx::FromRow, sqlx::Type, Serialize, Deserialize, Debug, Clone, PartialEq, Eq,
)]
#[sqlx(type_name = "RECORD")]
#[from_model(table_name = "my_adventures")]
pub struct MyAdventures {
    #[from_model(primary_key)]
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
    #[from_model(table_name = "my_users", model = "MyUsers", primary_key = "id")]
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

#[derive(
    FromModel, sqlx::FromRow, sqlx::Type, Serialize, Deserialize, Debug, Clone, PartialEq, Eq,
)]
#[sqlx(type_name = "RECORD")]
#[from_model(table_name = "my_adventures")]
pub struct NewMyAdventuresJourney {
    pub title: String,
    pub title_crypto: String,
    pub image_url: String,
    pub item_type: i16,
    pub link: String,
    pub source: i16,
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

#[derive(FromModel, Debug, Clone)]
#[from_model(table_name = "my_adventures")]
pub struct DeleteMyAdventure {
    #[from_model(primary_key)]
    pub id: ID,
}
