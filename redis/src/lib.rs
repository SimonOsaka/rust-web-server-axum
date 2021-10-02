pub mod connection;
pub mod operation;

use once_cell::sync::OnceCell;

use crate::connection::RedisConnection;
static REDIS: OnceCell<RedisConnection> = OnceCell::new();
#[macro_use]
extern crate log;
