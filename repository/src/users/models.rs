use macros::FromModel;
use serde::{Deserialize, Serialize};
use vars::{DateTime, ID};

#[derive(
    FromModel, sqlx::FromRow, Serialize, Deserialize, sqlx::Type, Debug, Clone, PartialEq, Eq,
)]
#[sqlx(type_name = "RECORD")]
#[from_model(table_name = "my_users")]
pub struct MyUsers {
    #[from_model(primary_key)]
    pub id: ID,
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
    pub is_deleted: i16,
    pub created_at: DateTime,
}

#[derive(FromModel, Debug)]
#[from_model(table_name = "my_users")]
pub struct NewMyUsers {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}
