///
/// User & user handlig
///
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::error::Error;
// use sqlx::FromRow;

/// User with status and description
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Option<i64>,
    pub email: Option<String>,
    pub password: Option<String>, // Note: Storing plain-text passwords is not recommended. Use hashed passwords instead.
    pub created_at: Option<DateTime<Local>>,
    pub openai_api_key: Option<String>,
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
