use thiserror::Error;
use time::error::{ComponentRange, Format, InvalidFormatDescription, Parse};

#[derive(Error, Debug)]
pub enum DateError {
    #[error("{}", "format date error.")]
    Format { e: Format },
    #[error("{}", "format date is out of range.")]
    FormatRange { e: ComponentRange },
    #[error("{}", "format pattern error.")]
    FormatInvalid {
        fmt: String,
        e: InvalidFormatDescription,
    },
    #[error("{}", "parse date error.")]
    ParseError { pattern: String, e: Parse },
    #[error("{}", "parse pattern {pattern} error.")]
    ParseInvalidPatternError {
        pattern: String,
        e: InvalidFormatDescription,
    },
}
