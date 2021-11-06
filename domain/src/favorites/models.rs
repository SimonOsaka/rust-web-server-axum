use repository::favorites::MyFavorites;
use serde::Serialize;
use types::ID;

#[derive(Serialize, Debug, Clone)]
pub struct Favorite {
    pub id: ID,
    pub user_id: ID,
    pub adventure_id: ID,
}

impl From<MyFavorites> for Favorite {
    fn from(f: MyFavorites) -> Self {
        Self {
            id: f.id,
            user_id: f.user_id,
            adventure_id: f.adventure_id,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct AddFavorite {
    pub user_id: ID,
    pub adventure_id: ID,
}

#[derive(Serialize, Debug, Clone)]
pub struct DelFavorite {
    pub user_id: ID,
    pub adventure_id: ID,
}
