use std::io::Error;

use xlsxwriter::XlsxError;

#[derive(thiserror::Error, Debug)]
pub enum ExcelError {
    #[error("{}", "export excel error.")]
    Export(XlsxError),
    #[error("{}", "io operation error")]
    IO(Error),
}
