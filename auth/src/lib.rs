#[cfg(any(feature = "authorization"))]
pub mod authorization;
#[cfg(any(feature = "jwt"))]
pub mod jwt;

#[cfg(any(feature = "authorization"))]
pub use authorization::*;
#[cfg(any(feature = "jwt"))]
pub use jwt::*;

/// init auth
pub async fn init() {
    #[cfg(any(feature = "authorization"))]
    init_authorization().await;
}
