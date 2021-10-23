use serde_derive::Serialize;
use warp::hyper::StatusCode;

#[derive(Debug, Clone)]
pub struct ErrorResponse(pub ErrorMessage, pub StatusCode);

impl warp::reject::Reject for ErrorResponse {}

impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        let error_message = self.0;
        let status_code = self.1;
        warp::reply::with_status(warp::reply::json(&error_message), status_code).into_response()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ErrorMessage {
    pub(crate) code: u16,
    pub(crate) message: String,
}
