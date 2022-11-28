use axum::{extract::State, Json};
use domain::{NewJourney, UsersManager};
use macros::router;
use serde::{Deserialize, Serialize};
use validator::Validate;
use vars::ID;

use crate::{
    app_request::{JwtAuth, ValidatedJson},
    app_response::AppError,
    AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct JourneyForm {
    #[validate(length(min = 5, max = 40, code = "adventure-journey-valid-title"))]
    title: String,

    #[validate(url(code = "adventure-journey-valid-image_url"))]
    image_url: String,

    #[validate(url(code = "adventure-journey-valid-link"))]
    link: String,

    #[validate(custom(function = "crate::app_request::validate_source"))]
    source: u8,

    #[validate(custom(function = "crate::app_request::validate_journey_destiny"))]
    journey_destiny: String,
}

#[derive(Serialize)]
pub struct JourneyResponse {
    id: ID,
}

#[tracing::instrument(skip(auth_user, state))]
#[router(path = "/api/adventures", method = "post")]
pub async fn journey(
    State(state): State<AppState>,
    JwtAuth(auth_user): JwtAuth,
    ValidatedJson(form): ValidatedJson<JourneyForm>,
) -> Result<Json<JourneyResponse>, AppError> {
    let adventures_manager = &state.adventures_manager;
    let users_manager = &state.users_manager;
    let user = users_manager
        .get_user_by_username(auth_user.get_name())
        .await?;
    let new_journey = NewJourney {
        title: form.title,
        image_url: form.image_url,
        link: form.link,
        source: form.source.into(),
        journey_destiny: form.journey_destiny,
    };
    let id = user.add_journey(new_journey, adventures_manager).await?;

    Ok(JourneyResponse { id }.into())
}
