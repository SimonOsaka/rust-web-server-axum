use domain::{AdventuresManagerImpl, UsersManagerImpl};
use std::{env, net::SocketAddr, sync::Arc};
use warp::{http::HeaderValue, Filter};

use crate::routes;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");
const IP_NONE: &str = "ip_none";

#[derive(Clone, Debug)]
pub struct AppStateRaw {
    pub adventures_manager: AdventuresManagerImpl,
    pub users_manager: UsersManagerImpl,
}

pub type AppState = Arc<AppStateRaw>;

pub async fn start() {
    let adventures_manager = AdventuresManagerImpl;
    let users_manager = UsersManagerImpl;
    let app_state = Arc::new(AppStateRaw {
        adventures_manager,
        users_manager,
    });
    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");
    // custom log format
    // pass x-real-ip from nginx
    let log = warp::log::custom(|info| {
        info!(
            target: APPLICATION_NAME,
            "{:?} \"{} {} {:?}\" {} \"{}\" \"{}\" {:?}",
            info.request_headers()
                .get("x-real-ip")
                .unwrap_or(&HeaderValue::from_static(IP_NONE)),
            info.method(),
            info.path(),
            info.version(),
            info.status().as_u16(),
            info.referer().unwrap_or("-"),
            info.user_agent().unwrap_or("-"),
            info.elapsed(),
        );
    });

    let routes = routes::routes(app_state).with(log);

    println!("You can access the server at {}", bind_address);

    warp::serve(routes).run(bind_address).await;
}
