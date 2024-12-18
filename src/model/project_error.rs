/// Spicific for service errors
// consider importing one of these structs: `use crate::StatusCode;

//use hyper::StatusCode;
//use reqwest::StatusCode;

use axum::http::StatusCode;

// --------------------------------------------------------
/// ERROR IN USE
// --------------------------------------------------------
//
//
#[derive(Debug)]
pub enum UserValidationError {
    MissingEmail,
    InvalidEmailFormat,
}

// --------------------------------------------------------
/// ERROR NOT IN USE
// at moment unstrucrured and unjoin with other code

enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorized,
    InternalServerError
}

fn internal_server_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
