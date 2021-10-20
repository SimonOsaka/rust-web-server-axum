use domain::manager_impl::Manager;
use std::{env, net::SocketAddr, sync::Arc};

use crate::app_routes;

#[derive(Clone, Debug)]
pub struct AppStateRaw {
    pub manager: Manager,
}

pub type AppState = Arc<AppStateRaw>;

pub async fn start() {
    let manager = Manager;
    let app_state = Arc::new(AppStateRaw { manager });
    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let routes = app_routes::routes(app_state);

    println!("listening on {}", bind_address);

    axum::Server::bind(&bind_address)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
