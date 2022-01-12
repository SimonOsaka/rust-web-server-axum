use i18n::i18n;
use std::borrow::Cow;

use validator::{ValidationError, ValidationErrors};

use crate::{
    my_item_type_format::to_item_type_name, my_journey_destiny::to_name, my_source::to_source_name,
};

pub fn to_new_validation_errors(e: ValidationErrors) -> ValidationErrors {
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
