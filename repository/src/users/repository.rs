use super::models::{MyUsers, NewMyUsers};
use crate::db::{SqlParams, SqlReader, SqlWriter};
use sql_builder::SqlBuilder;
use sqlx::Error;
use types::ID;

pub async fn insert(u: NewMyUsers) -> Result<ID, Error> {
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

    let id = sql_builder.insert_one(param).await?;
    debug!("insert id: {:?}", id);

    Ok(id)
}

pub async fn find_user_by_username(username: String) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from("my_users");
    sql_builder
        .fields(&[
            "id",
            "username",
            "created_at",
            "password",
            "roles",
            "is_deleted",
        ])
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.add_value(username));

    let my_users = sql_builder.query_one_optinal(param).await?;
    Ok(my_users)
}

pub async fn update_user_password(username: String, password: String) -> Result<bool, Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::update_table("my_users");
    sql_builder
        .set("password", param.add_value(password))
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.add_value(username));

    let affect = sql_builder.update_one(param).await?;
    Ok(affect > 0)
}

pub async fn update_username(old_username: String, new_username: String) -> Result<bool, Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::update_table("my_users");
    sql_builder
        .set("username", param.add_value(new_username))
        .and_where_eq("is_deleted", 0)
        .and_where_eq("username", param.add_value(old_username))
        .sql()
        .unwrap();
    let affect = sql_builder.update_one(param).await?;
    Ok(affect > 0)
}
