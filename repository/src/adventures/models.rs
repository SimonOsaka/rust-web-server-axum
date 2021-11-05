use types::{ID, U8I16};

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
    pub image_url: String,
    pub item_type: U8I16,
    pub link: String,
    pub source: U8I16,
    pub journey_destiny: String,
    pub user_id: ID,
}
