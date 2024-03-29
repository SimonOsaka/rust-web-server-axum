use crate::app_routes;
use domain::{
    adventures::AdventuresManagerImpl, favorites::FavoritesManagerImpl,
    UsersManagerImpl,
};
use std::{env, net::SocketAddr, sync::Arc};

#[derive(Clone, Debug)]
pub struct AppStateRaw {
    pub adventures_manager: AdventuresManagerImpl,
    pub users_manager: UsersManagerImpl,
    pub favorites_manager: FavoritesManagerImpl,
}

pub type AppState = Arc<AppStateRaw>;

pub async fn start() {
    let adventures_manager = AdventuresManagerImpl;
    let users_manager = UsersManagerImpl;
    let favorites_manager = FavoritesManagerImpl;
    let app_state = Arc::new(AppStateRaw {
        adventures_manager,
        users_manager,
        favorites_manager,
    });
    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let routes = app_routes::routes(app_state);

    println!("listening on {}", bind_address);

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
