use validator::Validate;
use warp::{reject::custom, Rejection};

use crate::{errors::ValidateError, response::ErrorResponse};

#[async_trait]
pub trait PathValidate: Validate {
    async fn valid(self) -> Result<Self, Rejection>
    where
        Self: Sized,
    {
        match self.validate() {
            Ok(_) => Ok(self),
            Err(e) => Err(custom(ErrorResponse::from(ValidateError::InvalidParam(e)))),
        }
    }
}
