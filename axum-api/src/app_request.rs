use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};

pub struct JwtToken(pub Option<String>);

#[async_trait]
impl<B> FromRequest<B> for JwtToken
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match req.headers() {
            Some(headers) => match headers.get("Authorization") {
                Some(token) => Ok(Self(Some(token.to_str().unwrap().to_string()))),
                None => Ok(Self(None)),
            },
            None => Ok(Self(None)),
        }
    }
}
