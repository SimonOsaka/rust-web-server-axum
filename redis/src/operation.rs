use redis::{aio::ConnectionManager, FromRedisValue, RedisError, ToRedisArgs};

/// Set redis key and value async
///
/// Example:
///
/// set<&str, String>("imkey", "imvalue").await
pub async fn set<K, V>(key: K, value: V) -> String
where
    K: ToRedisArgs,
    V: ToRedisArgs,
{
    let cm = &crate::REDIS.get().unwrap().connect_manager;
    let r = redis::cmd("SET")
        .arg(key)
        .arg(value)
        .query_async::<ConnectionManager, String>(&mut cm.clone())
        .await
        .unwrap();
    r
}

/// Set redis key and value async
///
/// Example:
///
/// set<&str, String>("imkey", "imvalue").await
pub async fn set_ex<K, S, V>(key: K, seconds: S, value: V) -> String
where
    K: ToRedisArgs,
    S: ToRedisArgs,
    V: ToRedisArgs,
{
    let cm = &crate::REDIS.get().unwrap().connect_manager;
    let r = redis::cmd("SETEX")
        .arg(key)
        .arg(seconds)
        .arg(value)
        .query_async::<ConnectionManager, String>(&mut cm.clone())
        .await
        .unwrap();
    debug!("setex result: {}", r);
    r
}

/// Get key's value async
///
/// Example:
///
/// get<&str, String>("imkey").await
pub async fn get<K, V>(key: K) -> Result<Option<V>, RedisError>
where
    K: ToRedisArgs,
    V: ToRedisArgs + FromRedisValue,
{
    let cm = &crate::REDIS.get().unwrap().connect_manager;
    let r = redis::cmd("GET")
        .arg(key)
        .query_async::<ConnectionManager, Option<V>>(&mut cm.clone())
        .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) => {
            warn!("{}", e);
            Err(e)
        }
    }
}

/// Del key's value async
///
/// Example:
///
/// del<&str>("imkey").await
pub async fn del<K>(key: K) -> Result<bool, RedisError>
where
    K: ToRedisArgs,
{
    let cm = &crate::REDIS.get().unwrap().connect_manager;
    match redis::cmd("DEL")
        .arg(key)
        .query_async::<ConnectionManager, bool>(&mut cm.clone())
        .await
    {
        Ok(v) => Ok(v),
        Err(e) => {
            warn!("{}", e);
            Err(e)
        }
    }
}

/// If key exist, Async
///
/// Example:
///
/// exist<&str>("imkey").await
pub async fn exist<K>(key: K) -> Result<bool, RedisError>
where
    K: ToRedisArgs,
{
    let cm = &crate::REDIS.get().unwrap().connect_manager;
    match redis::cmd("EXISTS")
        .arg(key)
        .query_async::<ConnectionManager, bool>(&mut cm.clone())
        .await
    {
        Ok(v) => Ok(v),
        Err(e) => {
            warn!("{}", e);
            Err(e)
        }
    }
}
