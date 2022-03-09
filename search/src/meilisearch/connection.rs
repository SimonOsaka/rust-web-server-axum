use std::env;

use meilisearch_sdk::client::Client;
use tracing::debug;

use crate::MEILISEARCH;

#[derive(Debug)]
pub struct MeiliSearch {
    pub(crate) adventures_client: Client,
}

impl MeiliSearch {
    /// create meilisearch client and index
    #[tracing::instrument]
    pub async fn create() {
        let meilisearch_url = env::var("MEILISEARCH_URL").expect("MEILISEARCH_URL must be set");
        let meilisearch_key = env::var("MEILISEARCH_KEY").expect("MEILISEARCH_KEY must be set");
        // create meilisearch client
        let adventures_client = Client::new(meilisearch_url, &meilisearch_key);
        //create meilisearch index
        let adventures_index = adventures_client.index("adventures_index");
        //create filterable_attributes for index
        let filterable_attributes = [
            "id",
            "play_list",
            "is_deleted",
            "item_type",
            "journey_destiny",
        ];
        adventures_index
            .set_filterable_attributes(filterable_attributes)
            .await
            .unwrap()
            .wait_for_completion(&adventures_client, None, None)
            .await
            .unwrap();
        // create sortable_attributes for index
        let sortable_attributes = ["id"];
        adventures_index
            .set_sortable_attributes(sortable_attributes)
            .await
            .unwrap()
            .wait_for_completion(&adventures_client, None, None)
            .await
            .unwrap();

        MEILISEARCH
            .set(MeiliSearch { adventures_client })
            .expect("meilisearch client must created");

        debug!("meilisearch connection created");
    }
}
