use crate::adventures::models::{AdventuresWhere, NewMyAdventuresJourney, PlayListWhere};
use crate::db::write::SqlWriter;
use crate::db::{SqlBuilder, SqlParams, SqlReader};
use crate::{AdventureUser, MyUsers, MY_USERS_STRUCT_FIELDS};

use sql_builder::{name, SqlName};
use sqlx::Error;
use types::{MyAdventures, ID};

const MY_ADVENTURES_FIELDS: &[&str; 17] = &[
    "ad.id",
    "ad.title",
    "ad.created_at",
    "ad.is_deleted",
    "ad.image_url",
    "ad.item_type",
    "ad.link",
    "ad.source",
    "ad.journey_destiny",
    "ad.script_content",
    "ad.play_list",
    "ad.address",
    "ad.shop_name",
    "ad.province",
    "ad.city",
    "ad.district",
    "ad.user_id",
];

const MY_ADVENTURES_STRUCT_FIELDS: &[&str; 17] = &[
    "(ad.id",
    "ad.title",
    "ad.image_url",
    "ad.created_at",
    "ad.is_deleted",
    "ad.item_type",
    "ad.link",
    "ad.source",
    "ad.journey_destiny",
    "ad.script_content",
    "ad.play_list",
    "ad.address",
    "ad.shop_name",
    "ad.province",
    "ad.city",
    "ad.district",
    "ad.user_id) AS \"my_adventures\"",
];

pub async fn find_latest(query: AdventuresWhere) -> Result<Vec<MyAdventures>, sqlx::Error> {
    let mut pgsql_builder = SqlBuilder::select_from(name!("my_adventures";"ad"));
    pgsql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq(name!("ad", "is_deleted"), 0);

    let mut param = SqlParams::new();

    if query.item_id != 0 {
        match query.province_key.as_ref() {
            // 字符串变量存在
            Some(pv) => {
                if pv.len() > 0 {
                    pgsql_builder
                        .and_where_eq(
                            name!("ad", "item_type"),
                            &param.add_value(query.item_id as i16),
                        )
                        .and_where_eq(
                            name!("ad", "journey_destiny"),
                            &param.add_value(query.province_key.unwrap()),
                        );
                } else {
                    pgsql_builder.and_where_eq(
                        name!("ad", "item_type"),
                        &param.add_value(query.item_id as i16),
                    );
                }
            }
            _ => {
                pgsql_builder.and_where_eq(
                    name!("ad", "item_type"),
                    &param.add_value(query.item_id as i16),
                );
            }
        }
    }

    pgsql_builder
        .order_desc(name!("ad", "id"))
        .limit(&param.add_value(query.limit.unwrap() as i64))
        .offset(&param.add_value(query.offset.unwrap() as i64));

    let my_adventures = pgsql_builder.query_list(param).await?;

    Ok(my_adventures)
}

pub async fn find_by_play_list(query: PlayListWhere) -> Result<Vec<MyAdventures>, sqlx::Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"ad"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq(name!("ad", "is_deleted"), 0)
        .and_where_eq(name!("ad", "play_list"), param.add_value(query.play_list));
    let my_adventures = sql_builder.query_list(param).await?;
    Ok(my_adventures)
}

pub async fn find_one(id: ID) -> Result<Option<MyAdventures>, sqlx::Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"ad"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq(name!("ad", "is_deleted"), 0)
        .and_where_eq(name!("ad", "id"), param.add_value(id as i64));

    let my = sql_builder.query_one_optinal(param).await?;
    Ok(my)
}

pub async fn create_journey(adventure: NewMyAdventuresJourney) -> Result<ID, sqlx::Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::insert_into("my_adventures");
    sql_builder
        .fields(&[
            "title",
            "title_crypto",
            "image_url",
            "item_type",
            "link",
            "source",
            "journey_destiny",
            "user_id",
        ])
        .values(&[
            param.add_value(adventure.title),
            param.add_value(adventure.title_crypto),
            param.add_value(adventure.image_url),
            param.add_value(adventure.item_type),
            param.add_value(adventure.link),
            param.add_value(adventure.source),
            param.add_value(adventure.journey_destiny),
            param.add_value(adventure.user_id),
        ])
        .returning_id();

    let id = sql_builder.insert_one(param).await?;
    debug!("insert id: {:?}", id);

    Ok(id)
}

pub async fn find_title_crypto(title_crypto: String) -> Result<Option<MyAdventures>, sqlx::Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"ad"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq(name!("ad", "is_deleted"), 0)
        .and_where_eq(name!("ad", "title_crypto"), param.add_value(title_crypto));

    let my = sql_builder.query_one_optinal(param).await?;
    Ok(my)
}

pub async fn delete_adventure(id: ID) -> Result<bool, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::update_table("my_adventures");
    sql_builder
        .set("is_deleted", 1)
        .and_where_eq("is_deleted", 0)
        .and_where_eq("id", params.add_value(id));

    let affect_rows = sql_builder.delete_one(params).await?;

    Ok(affect_rows == 1)
}

pub async fn find_by_user_id(user_id: ID) -> Result<Vec<(MyAdventures, MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"ad"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        .join(name!("my_users";"u"))
        .on("ad.user_id = u.id and u.is_deleted = 0")
        .and_where_eq(name!("ad", "is_deleted"), 0)
        .and_where_eq(name!("u", "id"), params.add_value(user_id));

    let list: Vec<AdventureUser> = //sql_builder.query_list_tuple().await?; 
    sql_builder.query_list(params).await?;

    let c = list
        .into_iter()
        .map(|adventure_user| {
            (
                adventure_user.my_adventures.into(),
                adventure_user.my_users.into(),
            )
        })
        .collect();
    Ok(c)
}
