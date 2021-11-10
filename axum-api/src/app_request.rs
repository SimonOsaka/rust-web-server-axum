use auth::{decode_token, Claims, JWTError};
use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Path, Query, RequestParts},
    BoxError, Json,
};
use http_body::Body;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::{app_error::ValidateError, app_response::AppError};

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
                None =>
                //Ok(Self(role_view())),
                {
                    Err(AppError::from(JWTError::Missing))
                }
            }
        } else {
            // Ok(Self(role_view()))
            Err(AppError::from(JWTError::Missing))
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    B: Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req)
            .await
            .map_err(|e| AppError::from(ValidateError::AxumQueryRejection(e)))?;
        value
            .validate()
            .map_err(|e| AppError::from(ValidateError::InvalidParam(e)))?;
        Ok(ValidatedQuery(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req)
            .await
            .map_err(|e| AppError::from(ValidateError::AxumJsonRejection(e)))?;
        value
            .validate()
            .map_err(|e| AppError::from(ValidateError::InvalidParam(e)))?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPath<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedPath<T>
where
    T: DeserializeOwned + Validate + Send,
    B: Send,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Path(value) = Path::<T>::from_request(req)
            .await
            .map_err(|e| AppError::from(ValidateError::AxumPathRejection(e)))?;
        value
            .validate()
            .map_err(|e| AppError::from(ValidateError::InvalidParam(e)))?;
        Ok(ValidatedPath(value))
    }
}
