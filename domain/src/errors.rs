#[derive(thiserror::Error, Debug)]
#[error("Something went wrong.")]
pub struct DomainError {
    #[from]
    source: anyhow::Error,
}
