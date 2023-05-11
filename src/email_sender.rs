use crate::error::{ApiError, ApiResult};
use async_trait::async_trait;
use lettre::{
    message::header::ContentType, message::Mailbox, transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use std::env;

pub struct Message {
    pub to: Mailbox,
    pub subject: String,
    pub body: String,
}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait EmailSender {
    async fn send_email(&self, message: Message) -> ApiResult<()>;
}

#[derive(Clone)]
pub struct RealEmailSender {}

impl RealEmailSender {
    pub fn new() -> Self {
        RealEmailSender {}
    }
}

lazy_static! {
    static ref EMAIL_ADDRESS: Mailbox = env::var("EMAIL_ADDRESS")
        .expect("set 'EMAIL_ADDRESS' environment variable.")
        .parse()
        .expect("'EMAIL_ADDRESS' is invalid");
    static ref EMAIL_PASSWORD: String =
        env::var("EMAIL_PASSWORD").expect("set 'EMAIL_PASSWORD' environment variable.");
}

#[async_trait]
impl EmailSender for RealEmailSender {
    async fn send_email(&self, message: Message) -> ApiResult<()> {
        let email = lettre::Message::builder()
            .from(EMAIL_ADDRESS.clone())
            .to(message.to)
            .subject(message.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(message.body)
            .unwrap();

        let creds = Credentials::new(EMAIL_ADDRESS.to_string(), EMAIL_PASSWORD.to_owned());

        // use 'starttls_relay' instead of 'relay' because it uses port 587 instead,
        // and this port is more likely to be open
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        mailer.send(email).await.map_err(|e| ApiError::EmailError {
            reason: e.to_string(),
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[actix_web::test]
    async fn send_real_test() {
        let email_sender = RealEmailSender::new();
        let message = Message {
            to: "change_this_to_real_email@gmail.com".parse().unwrap(),
            subject: "Test subject".to_string(),
            body: "This is body of test email".to_string(),
        };

        assert!(email_sender.send_email(message).await.is_ok())
    }
}
