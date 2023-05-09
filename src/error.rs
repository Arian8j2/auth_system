use actix_web::{http::StatusCode, ResponseError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    #[error("expired email code")]
    ExpiredEmailCode,

    #[error("wrong email code")]
    WrongEmailCode,

    #[error("user with same phone number already exists")]
    RegisterDuplicate,

    #[error("database error: {msg}")]
    SqlError { msg: String },

    #[error("argument '{argument_name}' is incorrect")]
    BadArgument { argument_name: &'static str },

    #[error("wrong credentials")]
    WrongCredentials,
}

pub type ApiResult<T> = Result<T, ApiError>;

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::SqlError { msg: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}
