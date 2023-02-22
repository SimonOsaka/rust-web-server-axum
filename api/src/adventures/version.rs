use crate::{adventures::response::VersionUpdateResponse, app_response::AppError};
use axum::{extract::Query, Json};
use macros::router;
use serde::Deserialize;
use tracing::debug;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct VersionUpdateReq {
    pub appid: String,
    pub version: String,
}

#[tracing::instrument]
#[router(path = "/api/adventures/update")]
pub async fn version_update_adventures(
    query: Query<VersionUpdateReq>,
) -> Result<Json<VersionUpdateResponse>, AppError> {
    debug!("query: {:?}", query);

    if query.appid != "__UNI__410C039" && query.appid != "HBuilder" {
        let response = VersionUpdateResponse {
            is_update: false,
            note: None,
            i_os: None,
            android: None,
        };
        Ok(response.into())
    } else {
        let mut is_update: bool = false;
        let mut note: Option<String> = None;
        let mut android: Option<String> = None;
        if query.version != "1.3.0" {
            is_update = true;
            note = Some("有新版本需要更新".to_string());
            android = Some("http://dl.jicu.vip/adventures_20201210.apk".to_string());
        }

        let response = VersionUpdateResponse {
            is_update,
            note,
            i_os: None,
            android,
        };
        debug!("response: {:?}", &response);
        Ok(response.into())
    }
}
