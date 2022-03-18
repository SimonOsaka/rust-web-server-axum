use axum::{
    body::{Bytes, Full},
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::json;
use util::Excel;

use crate::app_response::{AppError, ErrorMessage};

#[tracing::instrument]
pub async fn download() -> Result<impl IntoResponse, AppError> {
    let cols = vec!["cell1", "cell2"];
    let rows = vec![cols];
    let result = Excel::write_and_get_bytes(rows)?;

    let response_result = Response::builder()
        .header(
            "Content-type",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet; charset=utf-8",
        )
        .header("Content-Disposition", "attachment; filename=demo.xlsx")
        .body(Full::new(Bytes::from(result)));

    match response_result {
        Ok(res) => Ok(res),
        Err(e) => Err(AppError(
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(ErrorMessage {
                    message: format!("Unhandled internal error: {}", e),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                })),
            )
                .into_response(),
        )),
    }
}
