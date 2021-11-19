use super::{DeleteMyFavorite, GetMyFavorite, MyFavorites, NewMyFavorite};
use crate::db::{SqlParams, SqlReader, SqlWriter};
use sql_builder::{name, SqlBuilder, SqlName};
use sqlx::{Error, Postgres, Transaction};
use tracing::debug;
use types::ID;

#[tracing::instrument(skip(transaction), err)]
pub async fn insert<'a>(
    fav: NewMyFavorite,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<ID, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::insert_into("my_favorites");
    sql_builder
        .fields(&["user_id", "adventure_id"])
        .values(&[
            param.add_value(fav.user_id),
            param.add_value(fav.adventure_id),
        ])
        .returning_id();

    let id = sql_builder.insert_one(param, transaction).await?;
    debug!("insert id: {:?}", id);

    Ok(id)
}

#[tracing::instrument(skip(transaction), err)]
pub async fn delete<'a>(
    del: DeleteMyFavorite,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<bool, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::update_table("my_favorites");
    sql_builder
        .set("is_deleted", 1)
        .and_where_eq("is_deleted", 0)
        .and_where_eq("user_id", param.add_value(del.user_id))
        .and_where_eq("adventure_id", param.add_value(del.adventure_id));

    let affect_rows = sql_builder.delete_one(param, transaction).await?;
    debug!("delete affect_rows: {:?}", affect_rows);

    Ok(affect_rows > 0)
}

#[tracing::instrument(skip(transaction), err)]
pub async fn get_favorite<'a>(
    del: GetMyFavorite,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyFavorites>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));
    sql_builder
        .fields(&["fav.id", "fav.user_id", "fav.adventure_id"])
        .and_where_eq("fav.is_deleted", 0)
        .and_where_eq("fav.user_id", param.add_value(del.user_id))
        .and_where_eq("fav.adventure_id", param.add_value(del.adventure_id));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}
