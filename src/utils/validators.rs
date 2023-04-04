use crate::error::{ApiError, ApiResult};

pub fn validate_phone_number(phone_number: &str) -> ApiResult<()> {
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
