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
    email_sender: Data<dyn EmailSender + Send + Sync>,
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

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use super::*;
    use crate::{email_sender::MockEmailSender, test::helper::create_test_db};
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test::{self, TestRequest},
        App,
    };

    #[actix_web::test]
    async fn send_email_code_should_work() {
        let mut email_mock = MockEmailSender::new();
        email_mock
            .expect_send_email()
            .once()
            .returning(|_, _| Box::pin(ready(Ok(()))));
        let email_provider: Arc<dyn EmailSender + Send + Sync> = Arc::new(email_mock);

        let db = create_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(Data::new(db))
                .app_data(Data::from(email_provider))
                .service(send_email_code),
        )
        .await;
        let req = TestRequest::post()
            .uri("/send_email_code")
            .set_payload(r#"{"email_address": "arian@gmail.com"}"#)
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
