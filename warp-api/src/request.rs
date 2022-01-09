use auth::{decode_token, Claims, JWTError};
use serde::de::DeserializeOwned;
use types::to_new_validation_errors;
use validator::Validate;
use warp::{
    header::headers_cloned,
    hyper::{header::AUTHORIZATION, HeaderMap},
    query,
    reject::custom,
    Filter, Rejection, Reply,
};

use crate::{errors::ValidateError, response::ErrorResponse, AppState};

#[async_trait]
pub trait PathValidate: Validate {
    async fn valid(self) -> Result<Self, Rejection>
    where
        Self: Sized,
    {
        match self.validate() {
            Ok(_) => Ok(self),
            Err(e) => {
                let ves = to_new_validation_errors(e);
                Err(custom(ErrorResponse::from(ValidateError::InvalidParam(
                    ves,
                ))))
            }
        }
    }
}

#[derive(Debug)]
pub struct AuthUser(pub Claims);

pub fn with_auth() -> impl Filter<Extract = (AuthUser,), Error = Rejection> + Clone {
    headers_cloned().and_then(|x: HeaderMap| async move {
        match x.get(AUTHORIZATION) {
            Some(k) => match k.to_str().ok().and_then(|x| decode_token(x).ok()) {
                Some(k) => Ok(AuthUser(k)),
                None => Err(warp::reject::custom(ErrorResponse::from(JWTError::Invalid))),
            },
            // for no login user
            None =>
            //Ok(AuthUser(role_view())),
            {
                Err(warp::reject::custom(ErrorResponse::from(JWTError::Missing)))
            }
        }
    })
}

pub fn with_json_validate<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: Validate + std::marker::Send + DeserializeOwned,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json().and_then(
        |val: T| async move {
            match val.validate() {
                Ok(_) => Ok(val),
                Err(e) => {
                    let ves = to_new_validation_errors(e);
                    Err(custom(ErrorResponse::from(ValidateError::InvalidParam(
                        ves,
                    ))))
                }
            }
        },
    ))
}

pub fn with_query_validate<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: 'static + Validate + std::marker::Send + DeserializeOwned,
{
    query::<T>().and_then(|req: T| async move {
        match req.validate() {
            Ok(_) => Ok(req),
            Err(e) => {
                let ves = to_new_validation_errors(e);
                Err(custom(ErrorResponse::from(ValidateError::InvalidParam(
                    ves,
                ))))
            }
        }
    })
}

pub fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub async fn handle_rejection(
    err: warp::reject::Rejection,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    if let Some(e) = err.find::<ErrorResponse>() {
        return Ok(e.clone().into_response());
    }

    Err(err)
}
