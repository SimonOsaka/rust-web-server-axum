use repository::{
    db::Repo,
    favorites::{delete, get_favorite, insert, DeleteMyFavorite, GetMyFavorite, NewMyFavorite},
    find_one_adventure, update_adventure_fav, FavCountKind,
};
use search::adventures::add_adventure;

use crate::{
    database_to_domain_error, my_to_searched, search_to_domain_error, AddFavorite, DelFavorite,
    Favorite, FavoriteError,
};

#[derive(Clone, Debug)]
pub struct FavoritesManagerImpl;

#[async_trait]
impl super::FavoritesManager for FavoritesManagerImpl {
    #[tracing::instrument(skip(self))]
    async fn add_favorite(&self, add_favorite: AddFavorite) -> Result<Favorite, FavoriteError> {
        let mut transaction = Repo::transaction().await.expect("");

        let result_adventure =
            find_one_adventure(add_favorite.adventure_id, Some(&mut transaction)).await;
        if let Ok(result) = result_adventure {
            if result.is_none() {
                return Err(FavoriteError::AdventureNotFound {
                    adventure_id: add_favorite.adventure_id,
                });
            }
        }

        let result_optional = get_favorite(
            GetMyFavorite {
                user_id: add_favorite.user_id,
                adventure_id: add_favorite.adventure_id,
            },
            Some(&mut transaction),
        )
        .await;

        match result_optional {
            Ok(o) => {
                if let Some(my_fav) = o {
                    return Ok(Favorite::from(my_fav));
                }
            }
            Err(e) => return Err(FavoriteError::DomainError(database_to_domain_error(e))),
        }

        let inserted_id = insert(
            NewMyFavorite {
                user_id: add_favorite.user_id,
                adventure_id: add_favorite.adventure_id,
            },
            Some(&mut transaction),
        )
        .await
        .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        update_adventure_fav(
            add_favorite.adventure_id,
            FavCountKind::Fav,
            Some(&mut transaction),
        )
        .await
        .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        let result = find_one_adventure(add_favorite.adventure_id, Some(&mut transaction))
            .await
            .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        if let Some(my) = result {
            add_adventure(my_to_searched(my))
                .await
                .map_err(|e| FavoriteError::DomainError(search_to_domain_error(e)))?;
        }

        transaction.commit().await.expect("");

        Ok(Favorite {
            id: inserted_id,
            user_id: add_favorite.user_id,
            adventure_id: add_favorite.adventure_id,
        })
    }

    #[tracing::instrument(skip(self))]
    async fn del_favorite(&self, del_favorite: DelFavorite) -> Result<bool, FavoriteError> {
        let mut transaction = Repo::transaction().await.expect("");

        let result_adventure =
            find_one_adventure(del_favorite.adventure_id, Some(&mut transaction)).await;
        if let Ok(result) = result_adventure {
            if result.is_none() {
                return Err(FavoriteError::AdventureNotFound {
                    adventure_id: del_favorite.adventure_id,
                });
            }
        }

        let result_optional = get_favorite(
            GetMyFavorite {
                user_id: del_favorite.user_id,
                adventure_id: del_favorite.adventure_id,
            },
            Some(&mut transaction),
        )
        .await;

        match result_optional {
            Ok(o) => {
                if o.is_none() {
                    return Ok(true);
                }
            }
            Err(e) => return Err(FavoriteError::DomainError(database_to_domain_error(e))),
        }

        let success = delete(
            DeleteMyFavorite {
                user_id: del_favorite.user_id,
                adventure_id: del_favorite.adventure_id,
            },
            Some(&mut transaction),
        )
        .await
        .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        update_adventure_fav(
            del_favorite.adventure_id,
            FavCountKind::UnFav,
            Some(&mut transaction),
        )
        .await
        .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        let result = find_one_adventure(del_favorite.adventure_id, Some(&mut transaction))
            .await
            .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        if let Some(my) = result {
            add_adventure(my_to_searched(my))
                .await
                .map_err(|e| FavoriteError::DomainError(search_to_domain_error(e)))?;
        }

        transaction.commit().await.expect("");

        Ok(success)
    }
}
