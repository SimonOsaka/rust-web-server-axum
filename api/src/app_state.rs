use domain::manager_impl::Manager;
use std::{env, net::SocketAddr};
use warp::{http::HeaderValue, Filter};

use crate::routes;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");
const IP_NONE: &str = "ip_none";

#[derive(Clone, Debug)]
pub struct AppStateRaw {
    pub manager: Manager,
}

pub type AppState = std::sync::Arc<AppStateRaw>;

pub async fn start() {
    let manager = Manager;
    let app_state = std::sync::Arc::new(AppStateRaw { manager });
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
