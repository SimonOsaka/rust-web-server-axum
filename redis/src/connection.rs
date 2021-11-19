use std::{env, fmt};

use redis::aio::ConnectionManager;
use tracing::debug;

use crate::REDIS;

pub struct RedisConnection {
    pub(crate) connect_manager: ConnectionManager,
}

impl RedisConnection {
    async fn new(redis_url: &str) -> Self {
        let cm = redis::Client::open(redis_url)
            .unwrap()
            .get_tokio_connection_manager()
            .await
            .unwrap();
        Self {
            connect_manager: cm,
        }
    }

    pub async fn create() {
        let redis_url = env::var("REDIS_URL").unwrap();
        let redis_connection = RedisConnection::new(&redis_url);
        REDIS
            .set(redis_connection.await)
            .expect("redis conn must set");
        debug!("redis connection created");
    }
}

impl fmt::Debug for RedisConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedisConnection").finish()
    }
}
