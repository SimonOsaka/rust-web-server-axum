use auth::{decode_token, role_view, Claims, JWTError};
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};

use crate::app_response::AppError;

pub struct AuthUser(pub Claims);

#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Some(headers) = req.headers() {
            match headers.get("Authorization") {
                Some(k) => match k.to_str().ok().and_then(|x| decode_token(x).ok()) {
                    Some(k) => Ok(Self(k)),
                    None => Err(AppError::from(JWTError::Invalid)),
                },
                // for no login user
                None => Ok(Self(role_view())),
            }
        } else {
            Ok(Self(role_view()))
        }
    }
}
