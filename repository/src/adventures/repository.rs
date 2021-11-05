use crate::adventures::models::{AdventuresWhere, NewMyAdventuresJourney, PlayListWhere};
use crate::db::write::SqlWriter;
use crate::db::{SqlBuilder, SqlParams, SqlReader};

use types::{MyAdventures, ID};

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn find_latest(query: AdventuresWhere) -> Result<Vec<MyAdventures>, sqlx::Error> {
    let mut pgsql_builder = SqlBuilder::select_from("my_adventures");
    pgsql_builder
        .fields(&[
            "id",
            "title",
            "created_at",
            "is_deleted",
            "image_url",
            "item_type",
            "link",
            "source",
            "journey_destiny",
            "script_content",
            "play_list",
            "address",
            "shop_name",
            "province",
            "city",
            "district",
        ])
        .and_where_eq("is_deleted", 0);

    let mut param = SqlParams::new();

    if query.item_id != 0 {
        match query.province_key.as_ref() {
            // 字符串变量存在
            Some(pv) => {
                if pv.len() > 0 {
                    pgsql_builder
                        .and_where_eq("item_type", &param.add_value(query.item_id as i16))
                        .and_where_eq(
                            "journey_destiny",
                            &param.add_value(query.province_key.unwrap()),
                        );
                } else {
                    pgsql_builder.and_where_eq("item_type", &param.add_value(query.item_id as i16));
                }
            }
            _ => {
                pgsql_builder.and_where_eq("item_type", &param.add_value(query.item_id as i16));
            }
        }
    }

    pgsql_builder
        .order_desc("id")
        .limit(&param.add_value(query.limit.unwrap() as i64))
        .offset(&param.add_value(query.offset.unwrap() as i64));

    let my_adventures = pgsql_builder.query_list(param).await?;

    Ok(my_adventures)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn find_by_play_list(query: PlayListWhere) -> Result<Vec<MyAdventures>, sqlx::Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from("my_adventures");
    sql_builder
        .fields(&[
            "id",
            "title",
            "created_at",
            "is_deleted",
            "image_url",
            "item_type",
            "link",
            "source",
            "journey_destiny",
            "script_content",
            "play_list",
            "address",
            "shop_name",
            "province,city",
            "district",
        ])
        .and_where_eq("is_deleted", 0)
        .and_where_eq("play_list", param.add_value(query.play_list));
    let my_adventures = sql_builder.query_list(param).await?;
    Ok(my_adventures)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn find_one(id: ID) -> Result<Option<MyAdventures>, sqlx::Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from("my_adventures");
    sql_builder
        .fields(&[
            "id",
            "title",
            "created_at",
            "is_deleted",
            "image_url",
            "item_type",
            "link",
            "source",
            "journey_destiny",
            "script_content",
            "play_list",
            "address",
            "shop_name",
            "province",
            "city",
            "district",
        ])
        .and_where_eq("is_deleted", 0)
        .and_where_eq("id", param.add_value(id as i64));

    let my = sql_builder.query_one_optinal(param).await?;
    Ok(my)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
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

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn find_title_crypto(title_crypto: String) -> Result<Option<MyAdventures>, sqlx::Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from("my_adventures");
    sql_builder
        .fields(&[
            "id",
            "title",
            "created_at",
            "is_deleted",
            "image_url",
            "item_type",
            "link",
            "source",
            "journey_destiny",
            "script_content",
            "play_list",
            "address",
            "shop_name",
            "province",
            "city",
            "district",
        ])
        .and_where_eq("is_deleted", 0)
        .and_where_eq("title_crypto", param.add_value(title_crypto));

    let my = sql_builder.query_one_optinal(param).await?;
    Ok(my)
}
