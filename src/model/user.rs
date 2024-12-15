///
/// User & user handlig
///
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::error::Error;
// use sqlx::FromRow;

/// User with status and description with optional fields
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Option<i64>,
    pub email: Option<String>,
    /// [Note:] Storing plain-text passwords is not recommended.
    /// Use hashed passwords instead.
    pub password: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub openai_api_key: Option<String>,
}

/// User with normalized filds
/// - created_at: NaiveDateTime
/// The term "Naive UTC Date" refers to a simple, date-time representation without time zone information.
/// In Rust, the `chrono` crate, which is a popular library for handling date and time,
/// provides a `NaiveDateTime` type that represents dates and times in UTC without any additional timezone offset.
#[derive(Clone, Debug, sqlx::FromRow, Serialize)]
pub struct UserNormalized {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub openai_api_key: String,
}


#[derive(Debug)]
pub enum UserValidationError {
    MissingEmail,
    InvalidEmailFormat,
}

impl fmt::Display for UserValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEmail => write!(f, "Email is missing"),
            Self::InvalidEmailFormat => write!(f, "Invalid email format"),
        }
    }
}

impl Error for UserValidationError {}

pub fn verify_email(user_email: Option<String>) -> Result<String, UserValidationError> {
    match user_email {
        Some(email) => {
            if email.contains('@') && email.contains('.') {
                Ok(email)
            } else {
                Err(UserValidationError::InvalidEmailFormat)
            }
        },
        None => Err(UserValidationError::MissingEmail),
    }
}

/// # User 2 UserNormalized converter
/// if optionnal field = none than default value will be applied
fn convert_user(user: User) -> UserNormalized{
    let id: i64 = user.id.unwrap_or(0);
    let email: String = user.email.unwrap_or_else(|| "default_email@example.com".to_string());
    let password: String = user.password.unwrap_or_else(|| "default_password".to_string());

    // For DateTime, we need to handle None by converting it to a NaiveDateTime
    // let created_at = user.created_at.unwrap_or(NaiveDate::from_ymd(2023, 1, 1).and_hms(0, 0, 0));
    let created_at: NaiveDateTime = match user.created_at {
        Some(date)  => date,
        // here the date at first
        None =>  NaiveDate::from_ymd_opt(1900, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
    };

    let openai_api_key: String = user.openai_api_key
        .unwrap_or_else(|| "default_openai_api_key".to_string());

    UserNormalized {
        id,
        email,
        password,
        created_at,
        openai_api_key,
    }
}
