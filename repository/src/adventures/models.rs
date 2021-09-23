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

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct NewMyAdventures {
//     pub title: String,
//     pub image_url: String,
// }

// impl From<domain::AdventureContent> for NewMyAdventures {
//     fn from(content: domain::AdventureContent) -> Self {
//         Self {
//             title: content.title,
//             image_url: content.image_url,
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct UpdateMyAdventures {
//     pub id: ID,
//     pub title: String,
//     pub image_url: String,
// }

// impl From<domain::AdventuresUpdate> for UpdateMyAdventures {
//     fn from(update: domain::AdventuresUpdate) -> Self {
//         Self {
//             #[cfg(any(feature = "postgres"))]
//             id: update.id as i64,
//             #[cfg(any(feature = "mysql"))]
//             id: update.id,
//             title: update.title,
//             image_url: update.image_url,
//         }
//     }
// }

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
