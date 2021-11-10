use types::{MyAdventures, ID, U8I16};

use crate::MyUsers;

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
