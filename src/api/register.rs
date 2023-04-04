use crate::{
    db::{user::insert_user, DbPool, smscodes::get_last_sent_smscode},
    error::{ApiError, ApiResult},
    utils::validators::*,
    utils::hash::sha256_hash
};
use actix_web::{
    post,
    web::{Data, Json},
};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterArgs {
    phone_number: String,
    name: String,
    password: String,
    smscode: u32,
}

#[post("/register")]
pub async fn register(args: Json<RegisterArgs>, pool: Data<DbPool>) -> ApiResult<&'static str> {
    validate_phone_number(&args.phone_number)?;
    validate_name(&args.name)?;
    validate_password(&args.password)?;

    let sms_code = get_last_sent_smscode(&pool, &args.phone_number).await?;
    let now_date = Utc::now();
    if (sms_code.sent_date - now_date) > Duration::hours(1) {
        return Err(ApiError::ExpiredSmsCode)
    }

    if sms_code.code != args.smscode {
        return Err(ApiError::WrongSmsCode)
    }

    let hashed_password = sha256_hash(&args.password);
    insert_user(&pool, &args.name, &hashed_password, &args.phone_number).await?;
    Ok("")
}

#[cfg(test)]
mod tests {
    use crate::{test::helper::create_test_db, db::smscodes::insert_or_update_smscode};

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
        insert_or_update_smscode(&db, "09223768300", 123456).await.unwrap();

        let app = test::init_service(App::new().app_data(Data::new(db)).service(register)).await;
        let req = TestRequest::post()
            .uri("/register")
            .set_payload(
                r#"{"name": "arian", "password": "idkkkkl", "phone_number": "09223768300", "smscode": 123456}"#,
            )
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_same_phone_number_register() {
        let db = create_test_db().await;
        insert_or_update_smscode(&db, "09224769300", 789102).await.unwrap();
        let app = test::init_service(App::new().app_data(Data::new(db)).service(register)).await;

        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "arian", "password": "idkkk", "phone_number": "09224769300", "smscode": 789102}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "pouya", "password": "okkok", "phone_number": "09224769300", "smscode": 789102}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_invalid_phone_number_register() {
        let db = create_test_db().await;
        let app = test::init_service(App::new().app_data(Data::new(db)).service(register)).await;
        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "arian", "password": "idkkk", "phone_number": "0922", "smscode": 238218}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
