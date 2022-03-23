use validator::ValidationError;

use crate::{
    my_item_type_format::to_item_type_name, my_journey_destiny::to_name, my_source::to_source_name,
};

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
