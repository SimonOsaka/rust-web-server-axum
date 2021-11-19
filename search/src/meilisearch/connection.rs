use std::env;

use meilisearch_sdk::{client::Client, indexes::Index};
use tracing::debug;

use crate::MEILISEARCH;

#[derive(Debug)]
pub struct MeiliSearch {
    pub(crate) meili_key: String,
    pub(crate) adventures_index: Index,
}

impl MeiliSearch {
    /// create meilisearch client and index
    #[tracing::instrument]
    pub async fn create() {
        let meilisearch_url = env::var("MEILISEARCH_URL").expect("MEILISEARCH_URL must be set");
        let meilisearch_key = env::var("MEILISEARCH_KEY").expect("MEILISEARCH_KEY must be set");
        let ms_conn = Client::new(meilisearch_url, &meilisearch_key);
        let adventures_index = ms_conn.get_or_create("adventures_index").await.unwrap();

        MEILISEARCH
            .set(MeiliSearch {
                adventures_index,
                meili_key: meilisearch_key,
            })
            .expect("meilisearch index must created");

        debug!("meilisearch connection created");
    }
}
