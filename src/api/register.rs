use crate::{
    db::{email_codes::get_last_sent_email_code, user::insert_user, DbPool},
    error::{ApiError, ApiResult},
    utils::hash::sha256_hash,
    utils::validators::*,
};
use actix_web::{
    post,
    web::{Data, Json},
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterArgs {
    email_address: String,
    name: String,
    password: String,
    email_code: u32,
}

#[post("/register")]
pub async fn register(args: Json<RegisterArgs>, pool: Data<DbPool>) -> ApiResult<&'static str> {
    validate_email_address(&args.email_address)?;
    validate_name(&args.name)?;
    validate_password(&args.password)?;

    let email_code = get_last_sent_email_code(&pool, &args.email_address).await?;
    let now_date = Utc::now();
    if (email_code.sent_date - now_date) > Duration::hours(1) {
        return Err(ApiError::ExpiredEmailCode);
    }

    if email_code.code != args.email_code {
        return Err(ApiError::WrongEmailCode);
    }

    let hashed_password = sha256_hash(&args.password);
    insert_user(&pool, &args.name, &hashed_password, &args.email_address).await?;
    Ok("")
}

#[cfg(test)]
mod tests {
    use crate::{db::email_codes::insert_or_update_email_code, test::helper::create_test_db};

    use super::*;
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test,
        test::TestRequest,
        App,
    };

    #[actix_web::test]
    async fn test_register() {
        let db = create_test_db().await;
        insert_or_update_email_code(&db, "arian@gmail.com", 123456)
            .await
            .unwrap();

        let app = test::init_service(App::new().app_data(Data::new(db)).service(register)).await;
        let req = TestRequest::post()
            .uri("/register")
            .set_payload(
                r#"{"name": "arian", "password": "idkkkkl", "email_address": "arian@gmail.com", "email_code": 123456}"#,
            )
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_same_email_address_register() {
        let db = create_test_db().await;
        insert_or_update_email_code(&db, "arian@gmail.com", 789102)
            .await
            .unwrap();
        let app = test::init_service(App::new().app_data(Data::new(db)).service(register)).await;

        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "arian", "password": "idkkk", "email_address": "arian@gmail.com", "email_code": 789102}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "pouya", "password": "okkok", "email_address": "arian@gmail.com", "email_code": 789102}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_invalid_email_address_register() {
        let db = create_test_db().await;
        let app = test::init_service(App::new().app_data(Data::new(db)).service(register)).await;
        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "arian", "password": "idkkk", "email_address": "arian", "email_code": 238218}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
