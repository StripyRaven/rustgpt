/// LOCAL
use crate::model::{
    user::{
        User,
        UserNormalized
    },
    app_state::AppState
};

use axum::{
    extract::State,
    body::Body,
    http::{
        HeaderMap,
        HeaderValue,
        Request,
        StatusCode
    },
    middleware::Next,
    response::{
        Html,
        IntoResponse,
        Redirect,
        Response,
        Extension,
    },
};
use chrono::NaiveDateTime;
use std::sync::Arc;

use tera::Context;
use tower_cookies::Cookies;



pub fn error_response(code: u16, message: &str) -> Response {
    let to: String = format!("/error?code={}&message={}", code, message);
    let redirect: Redirect = Redirect::to(&to);
    let mut responce: Response<Body> = redirect.into_response();
    let h_map: &mut HeaderMap = responce.headers_mut();
    h_map.insert("HX-Redirect", HeaderValue::from_str(&to).unwrap());
    responce
}

/// # Extruct user
/// The `extract_user` function attempts to parse the user ID from cookies
/// and fetch the corresponding user from the database.
/// If any step fails, it returns a default value.
/// - [Note:] However, this approach can lead to potential issues if parsing or fetching operations fail unexpectedly.

pub async fn extract_user<B>(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode>
where
    B: Send + 'static,
{
    let session = cookies.get("rust-AI-session");

    let id: i64 = session.map_or(-1, |x| x.value()
        .parse::<i64>()
        .unwrap_or(-1));

    // Get the user
    match sqlx::query_as!(
        UserNormalized,
        "SELECT users.*, settings.openai_api_key \
        FROM users LEFT JOIN settings ON settings.user_id=users.id \
        WHERE users.id = $1",
        id
    )
    .fetch_one(&*state.pool)
    .await
    {
        Ok(current_user) => {
            // insert the current user into a request extension so the handler can
            // extract it, and make sure `user` is not used after this point
            req.extensions_mut().insert(Some(current_user));
            Ok(next.run(req).await)
        }
        Err(sqlx::Error::RowNotFound) => {
            // Handle specific error case for missing user
            req.extensions_mut().insert(None::<User>);
            // insert the current user into a request extension so the handler can
            // extract it, and make sure `user` is not used after this point
            Ok(next.run(req).await)
                },
        Err(_) => {
            eprintln!("Database fetch failed: {:?}", sqlx::Error::Io(std::io::Error::last_os_error()));
            Ok(error_response(500, "Internal Server Error"))
                }
    }
}

/// # Authentication Middleware (`auth`)
/// This middleware checks if the user is logged in and redirects to an error page if not.
pub async fn auth<B>(
    Extension(current_user): Extension<Option<User>>,
    request: Request<Body>,
    next: Next,
) -> Response
where
    B: Send + 'static,
{
    let to = format!("/error?code={}&message={}", "401", "Log in");
    let r = Redirect::to(&to);
    let mut r = r.into_response();
    let h = r.headers_mut();
    h.insert("HX-Redirect", HeaderValue::from_str(&to).unwrap());

    match current_user {
        Some(_user) => next.run(request).await,
        _ => error_response(401, "You need to log in to view this page"),
    }
}

pub async fn valid_openai_api_key<B>(
    Extension(current_user): Extension<Option<User>>,
    req: Request<Body>,
    next: Next,
) -> Response
where
    B: Send + 'static,
{
    let key = current_user
        .unwrap()
        .openai_api_key
        .unwrap_or(String::new());

    let client = reqwest::Client::new();
    match client
        .get("https://api.openai.com/v1/engines")
        .bearer_auth(&key)
        .send()
        .await
    {
        Ok(res) => {
            if res.status().is_success() {
                next.run(req).await
            } else {
                println!("failure!");
                error_response(403, "You API key is not set or invalid. Go to Settings.")
            }
        }
        Err(_) => error_response(403, "You API key is not set or invalid. Go to Settings"),
    }
}

//
pub async fn handle_error<B>(
    Extension(current_user): Extension<Option<User>>,
    State(state): State<Arc<AppState>>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode>
where
    B: Send + 'static,
{
    let response: Response<Body> = next.run(request).await;

    let status_code = response.status().as_u16();
    let status_text = response.status().as_str().to_string();

    match status_code {
        _ if status_code >= 400 => {
            let mut context = Context::new();
            context.insert("status_code", &status_code);
            context.insert("status_text", &status_text);

            let error = state.tera.render("views/error.html", &context).unwrap();

            let mut context = Context::new();
            context.insert("view", &error);
            context.insert("current_user", &current_user);
            context.insert("with_footer", &true);
            let rendered = state.tera.render("views/main.html", &context).unwrap();
            let h = Html(rendered).into_response();
            Ok(h)
        }
        _ => Ok(response),
    }
}
