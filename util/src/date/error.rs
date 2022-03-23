use crate::i18n::{i18n, i18n_with_vars};
use thiserror::Error;
use time::error::{ComponentRange, Format, InvalidFormatDescription, Parse};

#[derive(Error, Debug)]
pub enum DateError {
    #[error("{}", i18n("date-format-error"))]
    Format { e: Format },

    #[error("{}", i18n("date-out-of-range"))]
    FormatRange { e: ComponentRange },

    #[error("{}", i18n_with_vars("date-pattern-error", vec![fmt.to_string()]))]
    FormatInvalid {
        fmt: String,
        e: InvalidFormatDescription,
    },

    #[error("{}", i18n_with_vars("date-parse-error", vec![pattern.to_string()]))]
    ParseError { pattern: String, e: Parse },

    #[error("{}", i18n_with_vars("date-parse-pattern-error", vec![pattern.to_string()]))]
    ParseInvalidPatternError {
        pattern: String,
        e: InvalidFormatDescription,
    },
}
