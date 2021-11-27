use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("email address not correct")]
    Address { message: String },
    #[error("send email failed")]
    Send { message: String },
}
