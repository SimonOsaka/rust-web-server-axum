use std::borrow::Cow;

use crate::{
    app_error::{JWTError, ValidateError},
    app_response::AppError,
};
use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Path, Query, RequestParts},
    headers::{authorization::Bearer, Authorization},
    BoxError, Json, TypedHeader,
};
use http_body::Body;
use serde::de::DeserializeOwned;
use util::i18n::i18n;
use util::jwt::{decode_token, Claims};
use validator::{Validate, ValidationError, ValidationErrors};
use vars::{
    my_item_type_format::to_item_type_name, my_journey_destiny::to_name, my_source::to_source_name,
};

pub struct JwtAuth(pub Claims);

#[async_trait]
impl<B> FromRequest<B> for JwtAuth
where
    B: Send,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| JWTError::Invalid)?;

        match decode_token(bearer.token()) {
            Ok(k) => Ok(Self(k)),
            Err(_) => Err(AppError::from(JWTError::Invalid)),
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
        value.validate().map_err(|e| {
            let ves = to_new_validation_errors(e);
            AppError::from(ValidateError::InvalidParam(ves))
        })?;
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
        value.validate().map_err(|e| {
            let ves = to_new_validation_errors(e);
            AppError::from(ValidateError::InvalidParam(ves))
        })?;
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
        value.validate().map_err(|e| {
            let ves = to_new_validation_errors(e);
            AppError::from(ValidateError::InvalidParam(ves))
        })?;
        Ok(ValidatedPath(value))
    }
}

fn to_new_validation_errors(e: ValidationErrors) -> ValidationErrors {
    tracing::debug!("e.field_errors(): {:?}", e.field_errors());
    let mut new_validation_errors = ValidationErrors::new();
    for (field, vec_validation_error) in e.field_errors() {
        for validation_err in vec_validation_error {
            tracing::debug!("validation_err.code: {}", validation_err.code);
            let mut new_validation_error = validation_err.clone();
            new_validation_error.message = Some(Cow::from(i18n(
                new_validation_error.code.clone().into_owned().as_str(),
            )));
            new_validation_errors.add(field, new_validation_error);
        }
    }
    tracing::debug!(
        "ves.field_errors(): {:?}",
        new_validation_errors.field_errors()
    );

    new_validation_errors
}

pub fn validate_source(source: u8) -> Result<(), ValidationError> {
    if to_source_name(source.into()).is_empty() {
        return Err(ValidationError::new("adventure-journey-valid-source"));
    }

    Ok(())
}

pub fn validate_journey_destiny(journey_destiny: &str) -> Result<(), ValidationError> {
    if to_name(journey_destiny).is_empty() {
        return Err(ValidationError::new(
            "adventure-journey-valid-journey_destiny",
        ));
    }

    Ok(())
}

pub fn validate_item_id(item_id: u8) -> Result<(), ValidationError> {
    if to_item_type_name(item_id.into()).is_empty() {
        return Err(ValidationError::new("adventure-list-valid-item_id"));
    }

    Ok(())
}
