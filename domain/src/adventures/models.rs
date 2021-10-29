use repository::adventures::models::{AdventuresWhere, PlayListWhere};
use search::adventures::model::{AdventuresFilter, PlayListFilter};
use serde::Serialize;
use types::{DateTime, MyAdventures, ID, U8I16};

#[derive(Clone, Debug)]
pub struct AdventuresQuery {
    pub item_id: u8,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub province_key: Option<String>,
}

impl From<AdventuresWhere> for AdventuresQuery {
    fn from(w: AdventuresWhere) -> Self {
        Self {
            item_id: w.item_id,
            limit: w.limit,
            offset: w.offset,
            province_key: w.province_key,
        }
    }
}

impl Into<AdventuresWhere> for AdventuresQuery {
    fn into(self) -> AdventuresWhere {
        AdventuresWhere {
            item_id: (self.item_id),
            limit: (self.limit),
            offset: (self.offset),
            province_key: (self.province_key),
        }
    }
}

impl Into<AdventuresFilter> for AdventuresQuery {
    fn into(self) -> AdventuresFilter {
        AdventuresFilter {
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

impl From<MyAdventures> for Adventures {
    fn from(my: MyAdventures) -> Self {
        Adventures {
            id: my.id,
            title: my.title,
            image_url: my.image_url,
            created_at: my.created_at,
            item_type: my.item_type,
            link: my.link,
            source: my.source,
            journey_destiny: my.journey_destiny,
            script_content: my.script_content,
            play_list: my.play_list,
            address: my.address,
            shop_name: my.shop_name,
            province: my.province,
            city: my.city,
            district: my.district,
        }
    }
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

impl Into<PlayListFilter> for PlayListQuery {
    fn into(self) -> PlayListFilter {
        PlayListFilter {
            play_list: (self.play_list),
        }
    }
}
