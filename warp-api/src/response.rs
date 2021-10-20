use serde_derive::Serialize;
pub struct ErrorResponse(pub warp::reply::Response);

impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        self.0
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub(crate) code: u16,
    pub(crate) message: String,
}
