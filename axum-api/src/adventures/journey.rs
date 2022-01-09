use axum::{extract::Extension, Json};
use domain::{NewJourney, UsersManager};
use serde::{Deserialize, Serialize};
use types::my_journey_destiny::to_name;
use types::my_source::to_source_name;
use types::ID;
use validator::{Validate, ValidationError};

use crate::{
    app_request::{AuthUser, ValidatedJson},
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

    #[validate(custom(function = "validate_source"))]
    source: u8,

    #[validate(custom(function = "validate_journey_destiny"))]
    journey_destiny: String,
}

fn validate_source(source: u8) -> Result<(), ValidationError> {
    if to_source_name(source.into()) == "" {
        return Err(ValidationError::new("adventure-journey-valid-source"));
    }

    Ok(())
}

fn validate_journey_destiny(journey_destiny: &str) -> Result<(), ValidationError> {
    if to_name(&journey_destiny) == "" {
        return Err(ValidationError::new(
            "adventure-journey-valid-journey_destiny",
        ));
    }

    Ok(())
}

#[derive(Serialize)]
pub struct JourneyResponse {
    id: ID,
}

#[tracing::instrument(skip(auth_user, state))]
pub async fn journey(
    ValidatedJson(form): ValidatedJson<JourneyForm>,
    AuthUser(auth_user): AuthUser,
    Extension(state): Extension<AppState>,
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
