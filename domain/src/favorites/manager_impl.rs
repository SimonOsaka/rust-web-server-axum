use repository::favorites::{
    delete, get_favorite, insert, DeleteMyFavorite, GetMyFavorite, NewMyFavorite,
};

use crate::{
    database_to_domain_error, AddFavorite, DelFavorite, DomainError, Favorite, FavoriteError,
};

#[derive(Clone, Debug)]
pub struct FavoritesManagerImpl;

#[async_trait]
impl super::FavoritesManager for FavoritesManagerImpl {
    async fn add_favorite(&self, add_favorite: AddFavorite) -> Result<Favorite, FavoriteError> {
        let result_optional = get_favorite(GetMyFavorite {
            user_id: add_favorite.user_id,
            adventure_id: add_favorite.adventure_id,
        })
        .await;

        match result_optional {
            Ok(o) => {
                if o.is_some() {
                    let my_fav = o.unwrap();
                    return Ok(Favorite::from(my_fav));
                }
            }
            Err(e) => return Err(FavoriteError::DomainError(database_to_domain_error(e))),
        }

        let inserted_id = insert(NewMyFavorite {
            user_id: add_favorite.user_id,
            adventure_id: add_favorite.adventure_id,
        })
        .await
        .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        Ok(Favorite {
            id: inserted_id,
            user_id: add_favorite.user_id,
            adventure_id: add_favorite.adventure_id,
        })
    }

    async fn del_favorite(&self, del_favorite: DelFavorite) -> Result<bool, DomainError> {
        let result_optional = get_favorite(GetMyFavorite {
            user_id: del_favorite.user_id,
            adventure_id: del_favorite.adventure_id,
        })
        .await;

        match result_optional {
            Ok(o) => {
                if o.is_none() {
                    return Ok(true);
                }
            }
            Err(e) => return Err(database_to_domain_error(e)),
        }

        let result = delete(DeleteMyFavorite {
            user_id: del_favorite.user_id,
            adventure_id: del_favorite.adventure_id,
        })
        .await
        .map_err(database_to_domain_error)?;

        Ok(result)
    }
}
