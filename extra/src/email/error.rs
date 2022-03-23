use thiserror::Error;
use util::i18n::i18n_with_vars;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("{}", i18n_with_vars("email-address-not-correct", vec![message.to_string()]))]
    Address { message: String },
    #[error("{}", i18n_with_vars("email-send-failed", vec![message.to_string()]))]
    Send { message: String },
}
