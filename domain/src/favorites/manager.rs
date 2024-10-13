use std::future::Future;

use crate::favorites::{AddFavorite, DelFavorite, Favorite, FavoriteError};

pub trait FavoritesManager {
    fn add_favorite(
        &self,
        add_favorite: AddFavorite,
    ) -> impl Future<Output = Result<Favorite, FavoriteError>>;

    fn del_favorite(
        &self,
        del_favorite: DelFavorite,
    ) -> impl Future<Output = Result<bool, FavoriteError>>;
}
