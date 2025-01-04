#![allow(dead_code)]
///
/// User & user handlig
///
//Local
// use super::constant;
// use super::user_dto::UserDTO;
use super::project_error::UserValidationError;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

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
        }
        None => Err(UserValidationError::MissingEmail),
    }
}

fn get_created_at(user: &Use) -> NaiveDateTime {
    match user.created_at {
        Some(dt) => dt,
        None => NaiveDate::from_ymd_opt(1900, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    }
}
