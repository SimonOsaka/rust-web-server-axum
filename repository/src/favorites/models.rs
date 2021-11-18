use types::ID;

#[derive(sqlx::FromRow, Debug)]
pub struct MyFavorites {
    pub id: ID,
    pub user_id: ID,
    pub adventure_id: ID,
}

#[derive(Debug)]
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
