use super::models::{InsertMyUsers, MyUsers};
use crate::db::{insert_one, query_one, update_one, SqlParam};
use sql_builder::SqlBuilder;
use sqlx::Error;
use types::ID;

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn insert(u: InsertMyUsers) -> Result<ID, Error> {
    let mut param = SqlParam::new();

    let mut sql_builder = SqlBuilder::insert_into("my_users");
    sql_builder
        .fields(&["username", "password", "roles"])
        .values(&[
            param.value(u.username),
            param.value(u.password),
            param.value(u.roles),
        ])
        .returning_id();
    let sql = sql_builder.sql().unwrap();
    debug!("insert sql: {}", sql);

    let id = insert_one(&sql, param.fetch_args()).await?;
    debug!("insert id: {:?}", id);

    Ok(id)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn find_user_by_username(username: String) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParam::new();
    let sql = SqlBuilder::select_from("my_users")
        .fields(&[
            "id",
            "username",
            "created_at",
            "password",
            "roles",
            "is_deleted",
        ])
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.value(username))
        .sql()
        .unwrap();
    let my_users = query_one(&sql, param.fetch_args()).await?;
    Ok(my_users)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn find_user(username: String, password: String) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParam::new();
    let sql = SqlBuilder::select_from("my_users")
        .fields(&[
            "id",
            "username",
            "created_at",
            "password",
            "roles",
            "is_deleted",
        ])
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.value(username))
        .and_where_eq("password", param.value(password))
        .sql()
        .unwrap();
    let my_users = query_one(&sql, param.fetch_args()).await?;
    Ok(my_users)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn update_user_password(username: String, password: String) -> Result<bool, Error> {
    let mut param = SqlParam::new();
    let sql = SqlBuilder::update_table("my_users")
        .set("password", param.value(password))
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.value(username))
        .sql()
        .unwrap();
    let affect = update_one(&sql, param.fetch_args()).await?;
    Ok(affect > 0)
}

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn update_username(old_username: String, new_username: String) -> Result<bool, Error> {
    let mut param = SqlParam::new();
    let sql = SqlBuilder::update_table("my_users")
        .set("username", param.value(new_username))
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.value(old_username))
        .sql()
        .unwrap();
    let affect = update_one(&sql, param.fetch_args()).await?;
    Ok(affect > 0)
}
