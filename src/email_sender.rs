use crate::error::ApiResult;
use async_trait::async_trait;

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait EmailSender {
    async fn send_email(&self, to: &str, message: &str) -> ApiResult<()>;
}

#[derive(Clone)]
pub struct TempEmailSender {}

impl TempEmailSender {
    pub fn new() -> Self {
        TempEmailSender {}
    }
}

#[async_trait]
impl EmailSender for TempEmailSender {
    async fn send_email(&self, to: &str, message: &str) -> ApiResult<()> {
        println!("sent email to '{to}' with message '{message}'");
        Ok(())
    }
}
