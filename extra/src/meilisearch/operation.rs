use std::convert::TryInto;

use meilisearch_sdk::{client::Client, document::Document, search::SearchResult, tasks::Task};

use super::{error::MeiliSearchError, MEILISEARCH};

#[derive(Debug)]
pub enum MeiliSearchStatus {
    Succeeded,
    Failed,
}

fn get_client() -> &'static Client {
    let meilisearch = MEILISEARCH.get().unwrap();

    &meilisearch.adventures_client
}

/// add documents
pub async fn add_documents<T>(vec: Vec<T>) -> Result<MeiliSearchStatus, MeiliSearchError>
where
    T: Document,
{
    let meilisearch = MEILISEARCH.get().unwrap();

    let index = &meilisearch.adventures_client.index("adventures_index");

    let task = index.add_documents(&vec, Some("id")).await?;

    let status = task.wait_for_completion(get_client(), None, None).await?;

    let ms_status = match status {
        Task::Succeeded { .. } => MeiliSearchStatus::Succeeded,
        _ => MeiliSearchStatus::Failed,
    };

    Ok(ms_status)
}

/// delete documents
pub async fn del_documents<T>(uids: Vec<T>) -> Result<MeiliSearchStatus, MeiliSearchError>
where
    T: std::fmt::Display + serde::ser::Serialize + std::fmt::Debug,
{
    let meilisearch = MEILISEARCH.get().unwrap();

    let index = &meilisearch.adventures_client.index("adventures_index");

    let task = index.delete_documents(&uids).await?;

    let status = task.wait_for_completion(get_client(), None, None).await?;

    let ms_status = match status {
        Task::Succeeded { .. } => MeiliSearchStatus::Succeeded,
        _ => MeiliSearchStatus::Failed,
    };

    Ok(ms_status)
}

#[derive(Debug)]
pub struct Condition {
    pub filter: Option<String>,
    pub sort: Option<Sort>,
    pub page: Option<Page>,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            filter: None,
            sort: None,
            page: Some(Page {
                limit: 10,
                offset: 0,
            }),
        }
    }
}

impl Condition {
    pub fn new() -> Condition {
        Condition::default()
    }
}

#[derive(Clone, Debug)]
pub struct Sort {
    pub property: SortProperty,
    pub direct: SortDirect,
}
#[derive(Clone, Debug)]
pub enum SortProperty {
    ID,
}
#[derive(Clone, Debug)]
pub enum SortDirect {
    ASC,
    DESC,
}

#[derive(Debug)]
pub struct Page {
    pub limit: usize,
    pub offset: usize,
}
impl Page {
    pub fn one() -> Page {
        Page {
            limit: 1,
            offset: 0,
        }
    }
    pub fn of(index: usize) -> Page {
        Page {
            limit: 10,
            offset: 10 * (index - 1),
        }
    }
    pub fn from(limit: u32, offset: u32) -> Self {
        Page {
            limit: limit.try_into().unwrap(),
            offset: offset.try_into().unwrap(),
        }
    }
}
pub async fn search_documents_with_filter<T>(
    condition: Condition,
) -> Result<Vec<SearchResult<T>>, MeiliSearchError>
where
    T: 'static + Document,
{
    let meilisearch = MEILISEARCH.get().unwrap();

    let index = &meilisearch.adventures_client.index("adventures_index");

    let mut query = index.search();

    let filter = condition.filter;
    if let Some(ref f) = filter {
        query.with_filter(f.as_str());
    };
    if let Some(s) = condition.sort {
        match s.property {
            SortProperty::ID => match s.direct {
                SortDirect::DESC => query.with_sort(&["id:desc"]),
                SortDirect::ASC => query.with_sort(&["id:asc"]),
            },
        };
    }
    if let Some(page) = condition.page {
        query.with_limit(page.limit).with_offset(page.offset);
    }
    let search_results = query.execute().await?;
    Ok(search_results.hits)
}
