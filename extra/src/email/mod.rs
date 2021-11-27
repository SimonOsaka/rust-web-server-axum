pub mod error;

use std::time::Duration;

use lettre::{
    address::AddressError,
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use once_cell::sync::OnceCell;

use crate::email::error::EmailError;

static EMAIL: OnceCell<AsyncEmail> = OnceCell::new();

#[derive(Debug)]
pub struct AsyncEmail {
    pub smtp_pool: AsyncSmtpTransport<Tokio1Executor>,
}

#[derive(Debug)]
pub struct OneMail {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl OneMail {
    pub async fn send(self) -> Result<(), EmailError> {
        let from = std::env::var("SMTP_FROM").expect("SMTP_FROM failed");
        let email = Message::builder()
            .from(
                from.parse()
                    .map_err(|e: AddressError| EmailError::Address {
                        message: e.to_string(),
                    })
                    .unwrap(),
            )
            .to(self
                .to
                .parse()
                .map_err(|e: AddressError| EmailError::Address {
                    message: e.to_string(),
                })
                .unwrap())
            .subject(self.subject)
            .body(self.body)
            .unwrap();

        match EMAIL.get().unwrap().smtp_pool.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => Err(EmailError::Send {
                message: e.to_string(),
            }),
        }
    }
}

pub(crate) async fn email_init() {
    let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST failed");
    let smtp_port = std::env::var("SMTP_PORT").expect("SMTP_PORT failed");
    let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME failed");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD failed");

    let creds = Credentials::new(smtp_username, smtp_password);

    let smtp = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
        .unwrap()
        .port(smtp_port.parse::<u16>().expect("smtp port parse failed"))
        .credentials(creds)
        .pool_config(
            PoolConfig::new()
                .min_idle(1)
                .max_size(3)
                .idle_timeout(Duration::from_secs(30)),
        )
        .build();

    EMAIL
        .set(AsyncEmail { smtp_pool: smtp })
        .expect("Set EMAIL failed");
}
