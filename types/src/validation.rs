use i18n::i18n;
use std::borrow::Cow;

use validator::ValidationErrors;

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
