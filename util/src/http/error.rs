//! Error type for error handling

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

/// Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Error {
    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 422
    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// request error
    #[error("Http Request Error")]
    RequestError,
}
