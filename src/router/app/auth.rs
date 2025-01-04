#![allow(dead_code)]
// LOCAL
use crate::model::{app_state::AppStateProject, user_dto::UserDTO};

use axum::{
    // debug_handler,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
    Json,
};

use serde::Deserialize;
use std::sync::Arc;
use tera::Context;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

/**
 * # fn login
 * login
 * views/login.html
 * views/main.html
 *
 */
pub async fn login(State(state): State<Arc<AppStateProject>>) -> Html<String> {
    tracing::info!("Enter LOGIN");
    let mut context = Context::new();
    context.insert("name", "World");
    let home = state.tera_templates.render("views/login.html", &context).unwrap();

    let mut context = Context::new();
    context.insert("view", &home);
    let rendered = state.tera_templates.render("views/main.html", &context).unwrap();

    Html(rendered)
}

// TODO move to model
#[derive(Debug)]
pub enum LogInError {
    InvalidCredentials,
    DatabaseError(String),
}

impl IntoResponse for LogInError {
    fn into_response(self) -> Response {
        match self {
            LogInError::InvalidCredentials => {
                tracing::info!("Invalid Username or Password");
                (
                    StatusCode::BAD_REQUEST,
                    Json("Invalid Username or Password"),
                )
                    .into_response()
            }
            LogInError::DatabaseError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(message)).into_response()
            }
        }
    }
}

// TODO move to models
#[derive(Deserialize, Debug)]
pub struct LogInDTO {
    email: String,
    password: String,
}

/**
 * # fn login_form
 * login form
 *
 */
//#[debug_handler]
pub async fn login_form(
    cookies: Cookies,
    state: State<Arc<AppStateProject>>,
    Form(log_in): Form<LogInDTO>,
) -> Result<Redirect, LogInError> {
    tracing::info!("Enter LOGIN FORM");

    let user_db: UserDTO = sqlx::query_as!(
        UserDTO,
        r#"SELECT
            users.id,
            users.email,
            users.password,
            users.created_at,
            settings.openai_api_key
                FROM users
                LEFT JOIN settings
                ON settings.user_id=users.id
            WHERE users.email = $1"#,
        log_in.email,
    )
    .fetch_one(&*state.pool)
    .await
    .map_err(|_| LogInError::InvalidCredentials)?; // TODO DB e

    tracing::info!(
        "LOG IN DATA
        in email {}:
        in pass  {}:
        DTO pass {}:",
        log_in.email,
        log_in.password,
        user_db.password
    );

    if user_db.password != log_in.password {
        tracing::info!("password not match");
        return Err(LogInError::InvalidCredentials);
    }

    tracing::info!("LOG IN PASSED in FN");
    // [ ] TODO UserDTO to be inserted in shared env
    let cookie: Cookie = Cookie::build(("rust-ai-session", user_db.id.to_string()))
        // .domain("www.rust-lang.org")
        .path("/")
        // .secure(true)
        .http_only(true)
        .into();

    cookies.add(cookie);

    tracing::info!("GO TO MAIN :(/)");

    Ok(Redirect::to("/"))
}

/**
 * # fn signup
 * signup
 * views/signup.html
 * views/main.html
 *
 */
pub async fn signup(State(state): State<Arc<AppStateProject>>) -> Html<String> {
    tracing::info!("Enter SIGN UP");
    // TODO: Hash password
    let mut context = Context::new();
    context.insert("name", "World");
    let home = state.tera_templates.render("views/signup.html", &context).unwrap();

    let mut context = Context::new();
    context.insert("view", &home);
    let rendered = state.tera_templates.render("views/main.html", &context).unwrap();

    Html(rendered)
}

#[derive(Debug)]
pub enum SignUpError {
    PasswordMismatch,
    DatabaseError(String),
}

impl IntoResponse for SignUpError {
    fn into_response(self) -> Response {
        match self {
            SignUpError::PasswordMismatch => {
                (StatusCode::BAD_REQUEST, Json("Passwords do not match.")).into_response()
            }
            SignUpError::DatabaseError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(message)).into_response()
            }
        }
    }
}

// TODO move to models
/// # Sign υp
/// Struct (DTO in C#)
#[derive(Deserialize, Debug)]
pub struct SignUp {
    email: String,
    password: String,
    password_confirmation: String,
}

/// # FORM SIGNUP
/// [!note] By using explicit unwrapping, you can prevent any implicit
/// conversions that might be causing the issue with the `sqlx::query_as!`
/// macro. Make sure to apply this change in all relevant places
/// where `sqlx::query_as!` is used in your codebase.
#[axum::debug_handler]
pub async fn form_signup(
    state: State<Arc<AppStateProject>>,
    Form(sign_up): Form<SignUp>,
) -> Result<Redirect, SignUpError> {
    tracing::info!("Enter FORM SIGNUP");
    if sign_up.password != sign_up.password_confirmation {
        return Err(SignUpError::PasswordMismatch);
    }

    // insert into db
    match sqlx::query!(
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id",
        sign_up.email,
        sign_up.password
    )
    .fetch_one(&*state.pool)
    .await
    {
        Ok(_) => Ok(Redirect::to("/login")),
        // Handle database error, for example, a unique constraint violation
        Err(e) => {
            println!("{}", e);
            Err(SignUpError::DatabaseError(format!(
                "An error occurred while trying to sign up: {}",
                e
            )))
        }
    }
}

/**
 * # fn logout
 * logout
 * views/main.html
 */
#[axum::debug_handler]
pub async fn logout(cookies: Cookies) -> Result<Redirect, StatusCode> {
    tracing::info!("Start - logout");
    let logout_template = "/";
    let default_domain = "localhost";
    let session_name = "rust-ai-session";
    let mut cookie: Cookie = Cookie::build((session_name, ""))
        .domain(default_domain)
        .path(logout_template)
        // .secure(true)
        .http_only(true)
        .into();
    // .finish();

    cookie.make_removal();

    cookies.add(cookie);

    Ok(Redirect::to(&logout_template))
}
