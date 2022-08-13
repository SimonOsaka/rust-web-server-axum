use macros::FromModel;
use serde::{Deserialize, Serialize};
use vars::ID;

#[derive(FromModel, sqlx::FromRow, sqlx::Type, Serialize, Deserialize, Debug, Clone, PartialEq,Eq)]
#[sqlx(type_name = "RECORD")]
#[from_model(table_name = "my_favorites")]
pub struct MyFavorites {
    #[from_model(primary_key)]
    pub id: ID,
    pub user_id: ID,
    pub adventure_id: ID,
}

#[derive(FromModel, sqlx::FromRow, sqlx::Type, Serialize, Deserialize, Debug, Clone, PartialEq,Eq)]
#[sqlx(type_name = "RECORD")]
#[from_model(table_name = "my_favorites")]
pub struct NewMyFavorite {
    pub user_id: ID,
    pub adventure_id: ID,
}

#[derive(Debug)]
pub struct DeleteMyFavorite {
    pub user_id: ID,
    pub adventure_id: ID,
}

#[derive(Debug)]
pub struct GetMyFavorite {
    pub user_id: ID,
    pub adventure_id: ID,
}
