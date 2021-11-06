use super::{DeleteMyFavorite, GetMyFavorite, MyFavorites, NewMyFavorite};
use crate::db::{SqlParams, SqlReader, SqlWriter};
use sql_builder::SqlBuilder;
use sqlx::Error;
use types::ID;

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn insert(fav: NewMyFavorite) -> Result<ID, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::insert_into("my_favorites");
    sql_builder
        .fields(&["user_id", "adventure_id"])
        .values(&[
            param.add_value(fav.user_id),
            param.add_value(fav.adventure_id),
        ])
        .returning_id();

    let id = sql_builder.insert_one(param).await?;
    debug!("insert id: {:?}", id);

    Ok(id)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn delete(del: DeleteMyFavorite) -> Result<bool, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::update_table("my_favorites");
    sql_builder
        .set("is_deleted", 1)
        .and_where_eq("is_deleted", 0)
        .and_where_eq("user_id", param.add_value(del.user_id))
        .and_where_eq("adventure_id", param.add_value(del.adventure_id));

    let affect_rows = sql_builder.delete_one(param).await?;
    debug!("delete affect_rows: {:?}", affect_rows);

    Ok(affect_rows > 0)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn get_favorite(del: GetMyFavorite) -> Result<Option<MyFavorites>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from("my_favorites");
    sql_builder
        .fields(&["id", "user_id", "adventure_id"])
        .and_where_eq("is_deleted", 0)
        .and_where_eq("user_id", param.add_value(del.user_id))
        .and_where_eq("adventure_id", param.add_value(del.adventure_id));

    let res = sql_builder.query_one_optinal(param).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}
