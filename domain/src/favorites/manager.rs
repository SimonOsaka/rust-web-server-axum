use crate::{AddFavorite, DelFavorite, DomainError, Favorite, FavoriteError};

#[async_trait]
pub trait FavoritesManager {
    async fn add_favorite(&self, add_favorite: AddFavorite) -> Result<Favorite, FavoriteError>;

    async fn del_favorite(&self, del_favorite: DelFavorite) -> Result<bool, DomainError>;
}
