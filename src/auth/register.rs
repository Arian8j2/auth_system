use crate::{
    db::{user::insert_user, DbPool},
    error::{ApiError, ApiResult},
};
use actix_web::{
    post,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterArgs {
    phone_number: String,
    name: String,
    password: String,
}

#[post("/register")]
pub async fn register(args: Json<RegisterArgs>, pool: Data<DbPool>) -> ApiResult<&'static str> {
    validate_phone_number(&args.phone_number)?;
    validate_name(&args.name)?;
    insert_user(&pool, &args.name, &args.password, &args.phone_number).await?;
    Ok("")
}

fn validate_phone_number(phone_number: &str) -> ApiResult<()> {
    let has_valid_length = phone_number.len() == 11;
    let has_valid_characters = phone_number.chars().all(|char| char.is_ascii_digit());

    if has_valid_length && has_valid_characters {
        Ok(())
    } else {
        Err(ApiError::BadArgument { argument_name: "phone_number" })
    }
}

fn validate_name(name: &str) -> ApiResult<()> {
    if !name.chars().all(|char| char.is_ascii_alphanumeric()) {
        Err(ApiError::BadArgument { argument_name: "name" })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::helper::create_test_db;

    use super::*;
    use actix_web::{http::{header::ContentType, StatusCode}, test, test::TestRequest, App};

    #[actix_web::test]
    async fn test_register() {
        let db = create_test_db().await;
        let app = test::init_service(App::new().app_data(Data::new(db)).service(register)).await;
        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "arian", "password": "idk", "phone_number": "09223768300"}"#)
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_same_phone_number_register() {
        let db = create_test_db().await;
        let app = test::init_service(App::new().app_data(Data::new(db)).service(register)).await;

        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "arian", "password": "idk", "phone_number": "09224769300"}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "pouya", "password": "okk", "phone_number": "09224769300"}"#)
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
            .set_payload(r#"{"name": "arian", "password": "idk", "phone_number": "0922"}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
