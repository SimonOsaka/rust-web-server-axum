use std::convert::TryInto;

use meilisearch_sdk::{document::Document, errors::Error, search::SearchResult};

use crate::MEILISEARCH;

/// add documents
pub async fn add_documents<T>(vec: Vec<T>)
where
    T: Document,
{
    let meilisearch = MEILISEARCH.get().unwrap();

    let index = &meilisearch.adventures_index;

    index.add_documents(&vec, Some("id")).await.unwrap();
}

pub struct Condition {
    pub filter: Option<String>,
    pub sort: Option<Sort>,
    pub page: Option<Page>,
}
impl Condition {
    pub fn new() -> Condition {
        Condition {
            filter: None,
            sort: None,
            page: Some(Page {
                limit: 10,
                offset: 0,
            }),
        }
    }
}
#[derive(Clone)]
pub struct Sort {
    pub property: SortProperty,
    pub direct: SortDirect,
}
#[derive(Clone)]
pub enum SortProperty {
    ID,
}
#[derive(Clone)]
pub enum SortDirect {
    ASC,
    DESC,
}
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
) -> Result<Vec<SearchResult<T>>, Error>
where
    T: 'static + Document,
{
    let meilisearch = MEILISEARCH.get().unwrap();

    let index = &meilisearch.adventures_index;

    let mut query = index.search();

    let filter = condition.filter;
    if let Some(ref f) = filter {
        query.with_filter(&f.as_str());
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
    let search_results = query.execute().await.unwrap();

    let results = search_results.hits;

    Ok(results)
}
