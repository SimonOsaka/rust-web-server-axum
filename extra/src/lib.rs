#[cfg(any(feature = "email"))]
pub mod email;

pub async fn init() {
    #[cfg(any(feature = "email"))]
    email::email_init().await;
}
