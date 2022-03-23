use super::{
    error::SearchError,
    model::{AdventuresFilter, PlayListFilter},
};
use extra::meilisearch::operation::{
    search_documents_with_filter, Condition, Page, Sort, SortDirect, SortProperty,
};
use tracing::debug;
use vars::{MyAdventures, ID};

#[tracing::instrument(err)]
pub async fn search_latest(query: AdventuresFilter) -> Result<Vec<MyAdventures>, SearchError> {
    let mut filter = vec!["is_deleted = 0".to_string()];
    if query.item_id != 0 {
        filter.push(format!("item_type = {}", query.item_id));
    }
    if let Some(pv) = query.province_key {
        filter.push(format!("journey_destiny = {:?}", pv))
    }

    let mut condition = Condition::new();
    condition.filter = Some(filter.join(" AND "));
    condition.sort = Some(Sort {
        property: SortProperty::ID,
        direct: SortDirect::DESC,
    });
    condition.page = Some(Page::from(
        query.limit.unwrap_or(10),
        query.offset.unwrap_or(0),
    ));

    debug!("condition: {:?}", condition);

    let search_results = search_documents_with_filter::<MyAdventures>(condition).await;

    let result: Vec<MyAdventures> = search_results?.into_iter().map(|sr| sr.result).collect();
    Ok(result)
}

#[tracing::instrument(err)]
pub async fn search_by_play_list(query: PlayListFilter) -> Result<Vec<MyAdventures>, SearchError> {
    let mut condition = Condition::new();
    condition.filter = Some(format!(
        "play_list = {} AND is_deleted = 0",
        query.play_list
    ));
    condition.page = Some(Page::of(1));

    debug!("condition: {:?}", condition);

    let search_results = search_documents_with_filter::<MyAdventures>(condition).await;

    let result: Vec<MyAdventures> = search_results?.into_iter().map(|sr| sr.result).collect();

    Ok(result)
}

#[tracing::instrument(err)]
pub async fn search_one(id: ID) -> Result<Option<MyAdventures>, SearchError> {
    let mut condition = Condition::new();
    condition.filter = Some(format!("id = {} AND is_deleted = 0", id));
    condition.page = Some(Page::one());

    debug!("condition: {:?}", condition);

    let search_results = search_documents_with_filter::<MyAdventures>(condition).await;

    let result: Vec<MyAdventures> = search_results?.into_iter().map(|sr| sr.result).collect();

    if result.len() == 1 {
        Ok(Some(result.get(0).unwrap().to_owned()))
    } else {
        Ok(None)
    }
}
