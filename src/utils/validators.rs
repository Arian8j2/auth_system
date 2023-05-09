use crate::error::{ApiError, ApiResult};
use regex::Regex;

lazy_static! {
    static ref VALID_EMAIL_REGEX: Regex = {
        Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
            .expect("email validator regex is wrong")
    };
}

pub fn validate_email_address(email_address: &str) -> ApiResult<()> {
    VALID_EMAIL_REGEX
        .is_match(email_address)
        .then_some(())
        .ok_or(ApiError::BadArgument {
            argument_name: "email_address",
        })
}

pub fn validate_name(name: &str) -> ApiResult<()> {
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

pub fn validate_password(password: &str) -> ApiResult<()> {
    let has_valid_length = password.len() >= 5 || password.len() <= 64;
    if has_valid_length {
        Ok(())
    } else {
        Err(ApiError::BadArgument {
            argument_name: "password",
        })
    }
}
