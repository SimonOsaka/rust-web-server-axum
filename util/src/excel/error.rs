use crate::i18n::i18n;
use std::io::Error;

use xlsxwriter::XlsxError;

#[derive(thiserror::Error, Debug)]
pub enum ExcelError {
    #[error("{}({0})", i18n("excel-export-error"))]
    Export(XlsxError),
    #[error("{}({0})", i18n("excel-io-error"))]
    IO(Error),
}
