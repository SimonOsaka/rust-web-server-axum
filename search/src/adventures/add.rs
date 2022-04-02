use extra::meilisearch::operation::{add_documents, del_documents, MeiliSearchStatus};

use super::{error::SearchError, model::SearchedAdventures};

#[tracing::instrument]
pub async fn add_adventure(ad: SearchedAdventures) -> Result<bool, SearchError> {
    add_adventures(vec![ad]).await
}

#[tracing::instrument]
pub async fn add_adventures(ads: Vec<SearchedAdventures>) -> Result<bool, SearchError> {
    let status = add_documents(ads).await?;

    match status {
        MeiliSearchStatus::Succeeded => Ok(true),
        _ => Ok(false),
    }
}

#[tracing::instrument]
pub async fn delete_adventure(uid: i64) -> Result<bool, SearchError> {
    if uid < 1 {
        return Ok(false);
    }

    let status = del_documents(vec![uid]).await?;

    match status {
        MeiliSearchStatus::Succeeded => Ok(true),
        _ => Ok(false),
    }
}

#[tracing::instrument]
pub async fn delete_adventures(uids: Vec<i64>) -> Result<bool, SearchError> {
    let status = del_documents(uids).await?;

    match status {
        MeiliSearchStatus::Succeeded => Ok(true),
        _ => Ok(false),
    }
}
