use macros::router;

#[router(path = "/")]
pub async fn index() -> &'static str {
    "Hello world!"
}
