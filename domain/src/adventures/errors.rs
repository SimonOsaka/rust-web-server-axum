use crate::DatabaseError;

#[derive(thiserror::Error, Debug)]
pub enum GetAdventureError {
    #[error("There is no adventure with id {adventure_id:?}.")]
    NotFound {
        adventure_id: i64,
        source: DatabaseError,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}
