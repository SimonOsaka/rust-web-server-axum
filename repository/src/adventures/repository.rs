use crate::adventures::models::{AdventuresWhere, PlayListWhere};
use crate::db::{query_list, query_one, SqlBuilder, SqlParam};
use anyhow::Result;

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

    let mut param = SqlParam::new();

    if query.item_id != 0 {
        match query.province_key.as_ref() {
            // 字符串变量存在
            Some(pv) => {
                if pv.len() > 0 {
                    pgsql_builder
                        .and_where_eq("item_type", &param.value(query.item_id as i16))
                        .and_where_eq("journey_destiny", &param.value(query.province_key.unwrap()));
                } else {
                    pgsql_builder.and_where_eq("item_type", &param.value(query.item_id as i16));
                }
            }
            _ => {
                pgsql_builder.and_where_eq("item_type", &param.value(query.item_id as i16));
            }
        }
    }

    let sql = pgsql_builder
        .order_desc("id")
        .limit(&param.value(query.limit.unwrap() as i64))
        .offset(&param.value(query.offset.unwrap() as i64))
        .sql()
        .unwrap();
    debug!("sql:{}", &sql);

    let my_adventures = query_list(&sql, param.fetch_args()).await?;

    Ok(my_adventures)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn find_by_play_list(query: PlayListWhere) -> Result<Vec<MyAdventures>, sqlx::Error> {
    let mut param = SqlParam::new();
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
        .and_where_eq("play_list", param.value(query.play_list));
    let sql = sql_builder.sql().unwrap();
    let my_adventures = query_list(&sql, param.fetch_args()).await?;
    Ok(my_adventures)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn find_one(id: ID) -> Result<Option<MyAdventures>, sqlx::Error> {
    let mut param = SqlParam::new();
    let mut sql_builder = SqlBuilder::select_from("my_adventures");
    let sql = sql_builder
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
        .and_where_eq("id", param.value(id as i64))
        .sql()
        .unwrap();

    let my = query_one(&sql, param.fetch_args()).await?;
    Ok(my)
}
