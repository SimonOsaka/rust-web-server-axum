use thiserror::Error;
use time::error::{ComponentRange, Format, InvalidFormatDescription, Parse};

#[derive(Error, Debug)]
pub enum DateError {
    #[error("{}", "")]
    Format { e: Format },
    #[error("{}", "")]
    FormatRange { e: ComponentRange },
    #[error("{}", "")]
    FormatInvalid {
        fmt: String,
        e: InvalidFormatDescription,
    },
    #[error("{}", "")]
    ParseError { date_str: String, e: Parse },
    #[error("{}", "")]
    ParseInvalidFormatError {
        date_str: String,
        e: InvalidFormatDescription,
    },
}
