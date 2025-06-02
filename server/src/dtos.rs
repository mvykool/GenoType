use core::str;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use create::models::{User, UserRole};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]

pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,

    #[validate(
        length(min = 1, message = "Confirm password is required"),
        must_match(other = "password", message = "password do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}


