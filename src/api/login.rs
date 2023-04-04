use crate::{
    db::{user::does_user_exists, DbPool},
    error::{ApiError, ApiResult},
    utils::{hash::sha256_hash, validators::*},
};
use actix_web::{
    post,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginArgs {
    phone_number: String,
    password: String,
}

#[post("/login")]
pub async fn login(args: Json<LoginArgs>, pool: Data<DbPool>) -> ApiResult<&'static str> {
    validate_phone_number(&args.phone_number)?;
    validate_password(&args.password)?;
    let hashed_password = sha256_hash(&args.password);

    does_user_exists(&pool, &args.phone_number, &hashed_password)
        .await?
        .then_some("")
        .ok_or(ApiError::WrongCredentials)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::user::insert_user, test::helper::create_test_db};
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test::{self, TestRequest},
        App,
    };

    #[actix_web::test]
    async fn test_login() {
        let db = create_test_db().await;
        let phone_number = "09123231976";
        let password = sha256_hash("some_hard_password");
        insert_user(&db, "idk", &password, &phone_number)
            .await
            .unwrap();

        let app = test::init_service(App::new().app_data(Data::new(db)).service(login)).await;
        let req = TestRequest::post()
            .uri("/login")
            .set_payload(r#"{"phone_number": "09123231976", "password": "some_hard_password"}"#)
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_login_to_not_existed_account() {
        let db = create_test_db().await;
        let app = test::init_service(App::new().app_data(Data::new(db)).service(login)).await;
        let req = TestRequest::post()
            .uri("/login")
            .set_payload(r#"{"phone_number": "09123456789", "password": "some_hard_password"}"#)
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_login_with_wrong_password() {
        let db = create_test_db().await;
        let phone_number = "09123231976";
        let password = sha256_hash("some_hard_password");
        insert_user(&db, "idk", &password, &phone_number)
            .await
            .unwrap();

        let app = test::init_service(App::new().app_data(Data::new(db)).service(login)).await;
        let req = TestRequest::post()
            .uri("/login")
            .set_payload(r#"{"phone_number": "09123231976", "password": "another_password"}"#)
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
