use types::ID;

#[derive(sqlx::FromRow, Debug)]
pub struct MyFavorites {
    pub id: ID,
    pub user_id: ID,
    pub adventure_id: ID,
}

pub struct NewMyFavorite {
    pub user_id: ID,
    pub adventure_id: ID,
}

pub struct DeleteMyFavorite {
    pub user_id: ID,
    pub adventure_id: ID,
}

pub struct GetMyFavorite {
    pub user_id: ID,
    pub adventure_id: ID,
}
