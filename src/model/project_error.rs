#![allow(dead_code)]
#![allow(unused_imports)]
/// Spicific for service errors
/// consider importing one of these structs: `use crate::StatusCode;
/// [into response](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html)
/// [Body](https://docs.rs/axum/latest/axum/body/struct.Body.html)
use axum::{
    body::Body,
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Redirect, Response},
};

// --------------------------------------------------------
// ERROR IN USE
// --------------------------------------------------------

/**
   ## Error message
       unified of error message
   ### General Note
       There is a mix-up of `StatusCode` references on different traites, to align base in model used `u16` as most common base for every one
   - note: for conversions useful `from_u16` & `as_u16`
   - [Rust doc for StatusCode](https://docs.rs/http/1.2.0/http/status/struct.StatusCode.html)
*/
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}

impl ErrorMessage {
    /** Constructor for creating an `ErrorMessage` from a code and message
        ## Example 1
        ```rs
        use crate::model::project_error::ErrorMessage;

        #[tokio::main]
        async fn main() {
            let error_message = ErrorMessage::new(StatusCode::NOT_FOUND, "Not Found");
            let response = error_message.into_response();
            println!("{:?}", response);
        }
        ```
    */
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        ErrorMessage {
            code,
            message: message.into(),
        }
    }

    /** Convert the `ErrorMessage` into an `axum::response::Response`
        ```rs
        pub trait IntoResponse {
            // Required method
            fn into_response(self) -> Response<Body>;
        }
        ```
    [into_response](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html)
    */
    pub fn into_response(self) -> Response {
        //! used distinnc value for temlate
        let err_tmplate = "/error?code={1}&message={2}";
        let redirect_header = "HX-Redirect";
        // 1
        //let to = format!("/error?code={}&message={}", self.code, self.message);
        let to = err_tmplate
            .replace("{1}", self.code.to_string().as_str())
            .replace("{2}", self.message.as_str());
        let redirect = Redirect::to(&to);
        let mut response: Response = redirect.into_response();
        let header_map = response.headers_mut();
        header_map.insert(redirect_header, HeaderValue::from_str(&to).unwrap());
        //response
        // redirect.HeadersAppend([(
        //     HeaderValue::from_str("HX-Redirect").unwrap(),
        //     HeaderValue::from_str(&to).unwrap(),
        // )]);
        let res = response;
        res
    }
}

// --------------------------------------------------------
/// ERROR NOT IN USE
// at moment unstrucrured and unjoined with other code

#[derive(Debug)]
pub enum UserValidationError {
    MissingEmail,
    InvalidEmailFormat,
}

/// API error types for different scenarios.
#[derive(Debug)]
pub enum ApiError {
    BadRequest(ErrorMessage),
    Forbidden(ErrorMessage),
    Unauthorized(ErrorMessage),
    InternalServerError(ErrorMessage),
    DatabaseError(ErrorMessage),
}

impl From<axum::http::StatusCode> for ErrorMessage {
    fn from(status_code: axum::http::StatusCode) -> Self {
        ErrorMessage {
            code: status_code.as_u16(),
            message: format!("{}", status_code),
        }
    }
}

// TODO wrong
impl From<&str> for ErrorMessage {
    fn from(message: &str) -> Self {
        ErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    /** Convert the `ApiError` into an `axum::response::Response`
        ## Example 1
            ```rs
            use crate::model::project_error::ApiError;

            fn handle_api_error(api_error: ApiError) -> Response<Body> {
            api_error.into_response()
            }
            ```
        ## Example 𝟚
            ```rs
            use crate::model::project_error::ErrorMessage;

            #[tokio::main]
            async fn main() {
            let error_message = ErrorMessage::new(StatusCode::NOT_FOUND, "Not Found");
            let response = error_message.into_response();
            println!("{:?}", response);
            }
            ```
    */
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest(_error) => {
                ErrorMessage::from(StatusCode::BAD_REQUEST).into_response()
            }
            ApiError::Forbidden(_error) => {
                ErrorMessage::from(StatusCode::FORBIDDEN).into_response()
            }
            ApiError::Unauthorized(_error) => {
                ErrorMessage::from(StatusCode::UNAUTHORIZED).into_response()
            }
            ApiError::InternalServerError(_error) => {
                ErrorMessage::from(StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
            ApiError::DatabaseError(_error) => {
                ErrorMessage::from(StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        }
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`response
// TODO should use ErrorMessage
// TODO consder merge to `ErrorMessage` and `std::error::Error`
// there is no refs in project
//
fn internal_server_error<E>(err: E) -> impl IntoResponse
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        ErrorMessage::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string())
            .into_response(),
    )
}
