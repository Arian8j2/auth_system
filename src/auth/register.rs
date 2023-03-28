use crate::{
    db::{user::insert_user, DbPool},
    error::{ApiError, ApiResult},
};
use actix_web::{
    post,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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
    validate_password(&args.password)?;

    let hashed_password = sha256_hash(&args.password);
    insert_user(&pool, &args.name, &hashed_password, &args.phone_number).await?;
    Ok("")
}

fn validate_phone_number(phone_number: &str) -> ApiResult<()> {
    let has_valid_length = phone_number.len() == 11;
    let has_valid_characters = phone_number.chars().all(|char| char.is_ascii_digit());
    if has_valid_length && has_valid_characters {
        Ok(())
    } else {
        Err(ApiError::BadArgument {
            argument_name: "phone_number",
        })
    }
}

fn validate_name(name: &str) -> ApiResult<()> {
    let has_valid_length = name.len() > 2 || name.len() <= 16;
    let has_valid_characters = name.chars().all(|char| char.is_ascii_alphanumeric());
    if has_valid_characters && has_valid_length {
        Ok(())
    } else {
        Err(ApiError::BadArgument {
            argument_name: "name",
        })
    }
}

fn validate_password(password: &str) -> ApiResult<()> {
    let has_valid_length = password.len() >= 5 || password.len() <= 64;
    if has_valid_length {
        Ok(())
    } else {
        Err(ApiError::BadArgument {
            argument_name: "password",
        })
    }
}

fn sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use crate::test::helper::create_test_db;

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
        let app = test::init_service(App::new().app_data(Data::new(db)).service(register)).await;
        let req = TestRequest::post()
            .uri("/register")
            .set_payload(
                r#"{"name": "arian", "password": "idkkkkl", "phone_number": "09223768300"}"#,
            )
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
            .set_payload(r#"{"name": "arian", "password": "idkkk", "phone_number": "09224769300"}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = TestRequest::post()
            .uri("/register")
            .set_payload(r#"{"name": "pouya", "password": "okkok", "phone_number": "09224769300"}"#)
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
            .set_payload(r#"{"name": "arian", "password": "idkkk", "phone_number": "0922"}"#)
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_sha256_hash() {
        assert_eq!(
            sha256_hash("salam"),
            "0582bd2c13fff71d7f40ef5586e3f4da05a3a61fe5ba9f0b4d06e99905ab83ea"
        );
        assert_eq!(
            sha256_hash("hello are you ok #!?"),
            "d5ef5c1a3a959f846ae09ebe1472a51a7ae784a3f726457d8939e833f8f1d7ce"
        );
        assert_eq!(
            sha256_hash("12345Aa@&$hello%^"),
            "60fcd6b50b3d0d0bbf8d13ed5ff7e4b1844a1239fec1a94b0fe189222670e832"
        );
    }
}
