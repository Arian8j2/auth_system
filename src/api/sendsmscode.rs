use crate::{
    db::{smscodes::insert_or_update_smscode, DbPool},
    error::ApiResult,
    smsprovider::SmsProvider,
};
use actix_web::{
    post,
    web::{Data, Json},
};
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendSmsCodeArgs {
    phone_number: String,
}

#[post("/sendsmscode")]
pub async fn sendsmscode(
    args: Json<SendSmsCodeArgs>,
    smsprovider: Data<dyn SmsProvider>,
    pool: Data<DbPool>,
) -> ApiResult<&'static str> {
    let random_code = generate_random_six_digit_code();
    smsprovider
        .send_message(
            &format!("your register code is: {random_code}"),
            &args.phone_number,
        )
        .await?;

    insert_or_update_smscode(&pool, &args.phone_number, random_code).await?;
    Ok("")
}

fn generate_random_six_digit_code() -> u32 {
    let mut rng = rand::thread_rng();
    (0..=6)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u32>()
        .unwrap()
}

// TODO: write test
