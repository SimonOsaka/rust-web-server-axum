use repository::{
    adventures::update_adventure_fav,
    adventures::FavCountKind,
    adventures::MyAdventures,
    adventures::MyAdventuresFields,
    db::{
        types::{Operation, Value},
        Repo,
    },
    favorites::{MyFavorites, MyFavoritesFields, NewMyFavorite},
};
use search::adventures::add_adventure;

use crate::{
    adventures::my_to_searched, database_to_domain_error,
    favorites::AddFavorite, favorites::DelFavorite, favorites::Favorite,
    favorites::FavoriteError, search_to_domain_error,
};

#[derive(Clone, Debug)]
pub struct FavoritesManagerImpl;

#[async_trait]
impl super::FavoritesManager for FavoritesManagerImpl {
    #[tracing::instrument(skip(self))]
    async fn add_favorite(
        &self,
        add_favorite: AddFavorite,
    ) -> Result<Favorite, FavoriteError> {
        let mut transaction = Repo::transaction().await.expect("");

        let fields = vec![MyAdventuresFields::Id(Operation::Eq(Value::from(
            add_favorite.adventure_id,
        )))];
        let result_adventure =
            MyAdventures::get(fields, Some(&mut transaction)).await;
        if let Ok(result) = result_adventure {
            if result.is_none() {
                return Err(FavoriteError::AdventureNotFound {
                    adventure_id: add_favorite.adventure_id,
                });
            }
        }

        let fields = vec![
            MyFavoritesFields::UserId(Operation::Eq(Value::from(
                add_favorite.user_id,
            ))),
            MyFavoritesFields::AdventureId(Operation::Eq(Value::from(
                add_favorite.user_id,
            ))),
        ];
        let result_optional =
            MyFavorites::get(fields, Some(&mut transaction)).await;

        match result_optional {
            Ok(o) => {
                if let Some(my_fav) = o {
                    return Ok(Favorite::from(my_fav));
                }
            }
            Err(e) => {
                return Err(FavoriteError::DomainError(
                    database_to_domain_error(e),
                ))
            }
        }

        let inserted_id = NewMyFavorite {
            user_id: add_favorite.user_id,
            adventure_id: add_favorite.adventure_id,
        }
        .insert(Some(&mut transaction))
        .await
        .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        update_adventure_fav(
            add_favorite.adventure_id,
            FavCountKind::Fav,
            Some(&mut transaction),
        )
        .await
        .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        let fields = vec![MyAdventuresFields::Id(Operation::Eq(Value::from(
            add_favorite.adventure_id,
        )))];
        let result = MyAdventures::get(fields, Some(&mut transaction))
            .await
            .map_err(|e| {
                FavoriteError::DomainError(database_to_domain_error(e))
            })?;

        if let Some(my) = result {
            add_adventure(my_to_searched(my)).await.map_err(|e| {
                FavoriteError::DomainError(search_to_domain_error(e))
            })?;
        }

        transaction.commit().await.expect("");

        Ok(Favorite {
            id: inserted_id,
            user_id: add_favorite.user_id,
            adventure_id: add_favorite.adventure_id,
        })
    }

    #[tracing::instrument(skip(self))]
    async fn del_favorite(
        &self,
        del_favorite: DelFavorite,
    ) -> Result<bool, FavoriteError> {
        let mut transaction = Repo::transaction().await.expect("");

        let fields = vec![MyAdventuresFields::Id(Operation::Eq(Value::from(
            del_favorite.adventure_id,
        )))];
        let result_adventure =
            MyAdventures::get(fields, Some(&mut transaction)).await;
        if let Ok(result) = result_adventure {
            if result.is_none() {
                return Err(FavoriteError::AdventureNotFound {
                    adventure_id: del_favorite.adventure_id,
                });
            }
        }

        let fields = vec![
            MyFavoritesFields::UserId(Operation::Eq(Value::from(
                del_favorite.user_id,
            ))),
            MyFavoritesFields::AdventureId(Operation::Eq(Value::from(
                del_favorite.adventure_id,
            ))),
        ];
        let result_optional =
            MyFavorites::get(fields, Some(&mut transaction)).await;

        let success = match result_optional {
            Ok(ref o) => {
                if let Some(fav) = o {
                    fav.delete(Some(&mut transaction)).await.map_err(|e| {
                        FavoriteError::DomainError(database_to_domain_error(e))
                    })?
                } else {
                    return Ok(true);
                }
            }
            Err(e) => {
                return Err(FavoriteError::DomainError(
                    database_to_domain_error(e),
                ))
            }
        };

        update_adventure_fav(
            del_favorite.adventure_id,
            FavCountKind::UnFav,
            Some(&mut transaction),
        )
        .await
        .map_err(|e| FavoriteError::DomainError(database_to_domain_error(e)))?;

        let fields = vec![MyAdventuresFields::Id(Operation::Eq(Value::from(
            del_favorite.adventure_id,
        )))];
        let result = MyAdventures::get(fields, Some(&mut transaction))
            .await
            .map_err(|e| {
                FavoriteError::DomainError(database_to_domain_error(e))
            })?;

        if let Some(my) = result {
            add_adventure(my_to_searched(my)).await.map_err(|e| {
                FavoriteError::DomainError(search_to_domain_error(e))
            })?;
        }

        transaction.commit().await.expect("");

        Ok(success)
    }
}
