use serde::{Deserialize, Serialize};
use types::{
    my_date_format, my_item_type_format, my_journey_destiny, my_source, DateTime, ID, U8I16,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdventuresResponse {
    pub adventures: Vec<Adventures>,
    pub adventures_count: u64,
}

impl From<Vec<domain::Adventures>> for AdventuresResponse {
    fn from(ads: Vec<domain::Adventures>) -> Self {
        let adventures_count = ads.len() as u64;
        let adventures = ads
            .into_iter()
            .map(|ad| Adventures {
                id: ad.id,
                title: ad.title,
                image_url: ad.image_url,
                created_at: ad.created_at,
                item_type: ad.item_type,
                item_type_name: my_item_type_format::to_item_type_name(ad.item_type),
                link: ad.link,
                source: ad.source,
                source_name: my_source::to_source_name(ad.source),
                journey_destiny_name: my_journey_destiny::to_name(&ad.journey_destiny),
                script_content: ad.script_content,
                play_list: ad.play_list,
                address: ad.address,
                shop_name: ad.shop_name,
                province: ad.province,
                city: ad.city,
                district: ad.district,
                fav_count: ad.fav_count,
            })
            .collect();
        Self {
            adventures,
            adventures_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdventureResponse {
    pub adventure: Adventures,
}

impl From<domain::Adventures> for AdventureResponse {
    fn from(ad: domain::Adventures) -> Self {
        let adventure = Adventures {
            id: ad.id,
            title: ad.title,
            image_url: ad.image_url,
            created_at: ad.created_at,
            item_type: ad.item_type,
            item_type_name: my_item_type_format::to_item_type_name(ad.item_type),
            link: ad.link,
            source: ad.source,
            source_name: my_source::to_source_name(ad.source),
            journey_destiny_name: my_journey_destiny::to_name(&ad.journey_destiny),
            script_content: ad.script_content,
            play_list: ad.play_list,
            address: ad.address,
            shop_name: ad.shop_name,
            province: ad.province,
            city: ad.city,
            district: ad.district,
            fav_count: ad.fav_count,
        };
        Self { adventure }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Adventures {
    pub id: ID,
    pub title: String,
    pub image_url: String,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime,
    pub item_type: U8I16,
    pub item_type_name: String,
    pub link: String,
    pub source: U8I16,
    pub source_name: String,
    pub journey_destiny_name: String,
    pub script_content: String,
    pub play_list: String,
    pub address: String,
    pub shop_name: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub fav_count: i64,
}

impl From<domain::Adventures> for Adventures {
    fn from(ad: domain::Adventures) -> Self {
        Self {
            id: ad.id,
            title: ad.title,
            image_url: ad.image_url,
            created_at: ad.created_at,
            item_type: ad.item_type,
            item_type_name: my_item_type_format::to_item_type_name(ad.item_type),
            link: ad.link,
            source: ad.source,
            source_name: my_source::to_source_name(ad.source),
            journey_destiny_name: my_journey_destiny::to_name(&ad.journey_destiny),
            script_content: ad.script_content,
            play_list: ad.play_list,
            address: ad.address,
            shop_name: ad.shop_name,
            province: ad.province,
            city: ad.city,
            district: ad.district,
            fav_count: ad.fav_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response404 {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TabsResponse {
    pub tab_list: Vec<Tabs>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tabs {
    pub name: String,
    pub item_id: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionUpdateResponse {
    pub is_update: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MyAdventuresResponse {
    pub adventures: Vec<AdventureUser>,
    pub adventures_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    username: String,
}

impl From<domain::Users> for Users {
    fn from(u: domain::Users) -> Self {
        Self {
            username: u.username,
        }
    }
}

impl From<Vec<(domain::Adventures, domain::Users)>> for MyAdventuresResponse {
    fn from(vec: Vec<(domain::Adventures, domain::Users)>) -> Self {
        let adventures_count = vec.len() as u64;
        let adventures = vec
            .into_iter()
            .map(|domain_t| {
                let (domain_ad, domain_u) = domain_t;
                let adventure = Adventures::from(domain_ad);
                let user = Users::from(domain_u);
                AdventureUser { adventure, user }
            })
            .collect();

        Self {
            adventures,
            adventures_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdventureUser {
    pub adventure: Adventures,
    pub user: Users,
}
