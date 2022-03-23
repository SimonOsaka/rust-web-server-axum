use super::models::{MyUsers, NewMyUsers};
use crate::db::{SqlParams, SqlReader, SqlWriter};
use sql_builder::SqlBuilder;
use sqlx::{Error, Postgres, Transaction};
use tracing::debug;
use vars::ID;

const MY_USERS_FIELDS: &[&str; 6] = &[
    "id",
    "username",
    "created_at",
    "password",
    "roles",
    "is_deleted",
];

pub const MY_USERS_STRUCT_FIELDS: &[&str; 6] = &[
    "(u.id",
    "u.username",
    "u.password",
    "u.roles",
    "u.is_deleted",
    "u.created_at) AS \"my_users\"",
];

#[tracing::instrument(skip(transaction,u),fields(u.username=%u.username),err)]
pub async fn insert<'a>(
    u: NewMyUsers,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<ID, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::insert_into("my_users");
    sql_builder
        .fields(&["username", "password", "roles"])
        .values(&[
            param.add_value(u.username),
            param.add_value(u.password),
            param.add_value(u.roles),
        ])
        .returning_id();

    let id = sql_builder.insert_one(param, transaction).await?;
    debug!("insert id: {:?}", id);

    Ok(id)
}

#[tracing::instrument(skip(transaction), err)]
pub async fn find_user_by_username<'a>(
    username: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from("my_users");
    sql_builder
        .fields(MY_USERS_FIELDS)
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.add_value(username));

    let my_users = sql_builder.query_one_optinal(param, transaction).await?;
    Ok(my_users)
}

#[tracing::instrument(skip(transaction), err)]
pub async fn update_user_password<'a>(
    username: String,
    password: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<bool, Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::update_table("my_users");
    sql_builder
        .set("password", param.add_value(password))
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.add_value(username));

    let affect = sql_builder.update_one(param, transaction).await?;
    Ok(affect > 0)
}

#[tracing::instrument(skip(transaction), err)]
pub async fn update_username<'a>(
    old_username: String,
    new_username: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<bool, Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::update_table("my_users");
    sql_builder
        .set("username", param.add_value(new_username))
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.add_value(old_username))
        .sql()
        .unwrap();
    let affect = sql_builder.update_one(param, transaction).await?;
    Ok(affect > 0)
}
