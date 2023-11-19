use repository::{
    adventures::models::{AdventuresWhere, PlayListWhere},
    adventures::MyAdventures,
    adventures::NewMyAdventuresJourney,
};
use search::adventures::model::{
    AdventuresFilter, PlayListFilter, SearchedAdventures,
};
use serde::Serialize;
use vars::{DateTime, ID};

use crate::{utils::hash, Users};

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

impl From<AdventuresQuery> for AdventuresWhere {
    fn from(aq: AdventuresQuery) -> Self {
        AdventuresWhere {
            item_id: (aq.item_id),
            limit: (aq.limit),
            offset: (aq.offset),
            province_key: (aq.province_key),
        }
    }
}

impl From<AdventuresQuery> for AdventuresFilter {
    fn from(aq: AdventuresQuery) -> Self {
        AdventuresFilter {
            item_id: (aq.item_id),
            limit: (aq.limit),
            offset: (aq.offset),
            province_key: (aq.province_key),
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Adventures {
    pub id: ID,
    pub title: String,
    pub image_url: String,
    pub created_at: DateTime,
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
            user_id: my.user_id,
            fav_count: my.fav_count,
        }
    }
}

impl From<SearchedAdventures> for Adventures {
    fn from(sa: SearchedAdventures) -> Self {
        Adventures {
            id: sa.id,
            title: sa.title,
            image_url: sa.image_url,
            created_at: sa.created_at,
            item_type: sa.item_type,
            link: sa.link,
            source: sa.source,
            journey_destiny: sa.journey_destiny,
            script_content: sa.script_content,
            play_list: sa.play_list,
            address: sa.address,
            shop_name: sa.shop_name,
            province: sa.province,
            city: sa.city,
            district: sa.district,
            user_id: sa.user_id,
            fav_count: sa.fav_count,
        }
    }
}

pub fn my_to_searched(ma: MyAdventures) -> SearchedAdventures {
    let MyAdventures {
        id,
        title,
        image_url,
        created_at,
        is_deleted,
        item_type,
        link,
        source,
        journey_destiny,
        script_content,
        play_list,
        address,
        shop_name,
        province,
        city,
        district,
        user_id,
        fav_count,
    } = ma;

    SearchedAdventures {
        id,
        title,
        image_url,
        created_at,
        is_deleted,
        item_type,
        link,
        source,
        journey_destiny,
        script_content,
        play_list,
        address,
        shop_name,
        province,
        city,
        district,
        user_id,
        fav_count,
    }
}

#[derive(Clone, Debug)]
pub struct PlayListQuery {
    pub play_list: String,
}

impl From<PlayListQuery> for PlayListWhere {
    fn from(plq: PlayListQuery) -> Self {
        PlayListWhere {
            play_list: (plq.play_list),
        }
    }
}

impl From<PlayListQuery> for PlayListFilter {
    fn from(plq: PlayListQuery) -> Self {
        PlayListFilter {
            play_list: (plq.play_list),
        }
    }
}

#[derive(Debug)]
pub struct NewJourney {
    pub title: String,
    pub image_url: String,
    pub link: String,
    pub source: i16,
    pub journey_destiny: String,
}

impl NewJourney {
    pub fn crypto(&self) -> String {
        hash(self.title.clone())
    }
}

#[derive(Debug)]
pub struct NewJourneyData {
    pub nj: NewJourney,
    pub u: Users,
}

impl From<NewJourneyData> for NewMyAdventuresJourney {
    fn from(data: NewJourneyData) -> Self {
        Self {
            title: data.nj.title.to_owned(),
            title_crypto: data.nj.crypto(),
            image_url: data.nj.image_url,
            item_type: 5,
            link: data.nj.link,
            source: data.nj.source,
            journey_destiny: data.nj.journey_destiny,
            user_id: data.u.id,
        }
    }
}
