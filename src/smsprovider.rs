use crate::error::ApiResult;
use async_trait::async_trait;

#[async_trait]
pub trait SmsProvider {
    async fn send_message(&self, msg: &str, target_phone_number: &str) -> ApiResult<()>;
}

#[derive(Clone)]
pub struct TempSmsProvider {}

impl TempSmsProvider {
    pub fn new() -> Self {
        TempSmsProvider {}
    }
}

#[async_trait]
impl SmsProvider for TempSmsProvider {
    async fn send_message(&self, msg: &str, target_phone_number: &str) -> ApiResult<()> {
        println!("sent sms '{msg}' -> '{target_phone_number}'");
        Ok(())
    }
}
