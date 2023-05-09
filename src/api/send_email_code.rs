use crate::{
    db::{email_codes::insert_or_update_email_code, DbPool},
    email_sender::EmailSender,
    error::ApiResult,
    utils::random::generate_random_six_digit_code,
};
use actix_web::{
    post,
    web::{Data, Json},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendEmailCodeArgs {
    email_address: String,
}

#[post("/send_email_code")]
pub async fn send_email_code(
    args: Json<SendEmailCodeArgs>,
    email_sender: Data<dyn EmailSender>,
    pool: Data<DbPool>,
) -> ApiResult<&'static str> {
    let random_code = generate_random_six_digit_code();
    email_sender
        .send_email(
            &format!("your register code is: {random_code}"),
            &args.email_address,
        )
        .await?;

    insert_or_update_email_code(&pool, &args.email_address, random_code).await?;
    Ok("")
}
