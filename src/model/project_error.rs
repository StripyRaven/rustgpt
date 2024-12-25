/// Spicific for service errors
// consider importing one of these structs: `use crate::StatusCode;

//use hyper::StatusCode;
//use reqwest::StatusCode;
use axum::{
    //async_trait,
    //extract::State,
    body::Body,
    http::{
        //HeaderMap,
        HeaderValue,
        //Request,
        StatusCode,
    },
    //middleware::Next,
    response::{
        //Extension,
        //User,
        //Html,
        IntoResponse,
        Redirect,
        Response,
    },
};
//use http::status::StatusCode; // containeed in axum
use serde::Deserialize;

// --------------------------------------------------------
/// ERROR IN USE
// --------------------------------------------------------

/// # Error message
/// unified of error message
/// ### General Note
/// There is a mix-up of `StatusCode` references on different traites, to align base in model used `u16` as most common base for every one
/// for conversions useful `from_u16` & `as_u16`
///  - [Rust doc for StatusCode](https://docs.rs/http/1.2.0/http/status/struct.StatusCode.html)
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}

impl ErrorMessage {
    /// Constructor for creating an `ErrorMessage` from a code and message
    /// ## Example 1
    /// ```rs
    ///use crate::model::project_error::ErrorMessage;
    ///
    ///    #[tokio::main]
    /// async fn main() {
    ///  let error_message = ErrorMessage::new(StatusCode::NOT_FOUND, "Not Found");
    ///let response = error_message.into_response();
    ///        println!("{:?}", response);
    /// }
    ///    ```
    ///
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        ErrorMessage {
            code,
            message: message.into(),
        }
    }

    /// Convert the `ErrorMessage` into an `axum::response::Response`
    pub fn into_response(self) -> Response<Body> {
        let to = format!("/error?code={}&message={}", self.code, self.message);
        let redirect = Redirect::to(&to);
        let mut response: Response<Body> = redirect.into_response();
        let h_map = response.headers_mut();
        h_map.insert("HX-Redirect", HeaderValue::from_str(&to).unwrap());
        response
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
}

impl IntoResponse for ApiError {
    /// Convert the `ApiError` into an `axum::response::Response`
    ///## Example 1
    /// ```rs
    /// use crate::model::project_error::ApiError;
    ///
    /// fn handle_api_error(api_error: ApiError) -> Response<Body> {
    ///    api_error.into_response()
    ///}
    ///```
    /// ## Example 𝟚
    /// ```rs
    ///use crate::model::project_error::ErrorMessage;
    ///
    ///#[tokio::main]
    /// async fn main() {
    ///   let error_message = ErrorMessage::new(StatusCode::NOT_FOUND, "Not Found");
    ///   let response = error_message.into_response();
    ///    println!("{:?}", response);
    ///}
    ///```

    fn into_response(self) -> Response<Body> {
        match self {
            ApiError::BadRequest(error) => error.into_response(),
            ApiError::Forbidden(error) => error.into_response(),
            ApiError::Unauthorized(error) => error.into_response(),
            ApiError::InternalServerError(error) => error.into_response(),
        }
    }

    // fn into_response(self) -> Response<Body> {
    //         self.0.into_response()
    //     }
}

// TODO should use ErrorMessage
// TODO consder merge to `ErrorMessage` and `std::error::Error`
fn internal_server_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
