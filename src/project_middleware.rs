/// LOCAL
use crate::model::{
    user_dto,
    user_dto::UserDTO,
    app_state::AppState};

use axum::{
    //async_trait,
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
        // client::Client,
        Html,
        IntoResponse,
        Redirect,
        Response,
        Extension,
        //User,
    },
};
//use::furure::Future;

use std::sync::Arc; // Sqlx use Arq, check via tree
use sqlx::Error as SqlError;

use tera::Context;
use tower_cookies::Cookies;



pub fn error_response(code: u16, message: &str) -> Response {
    let to: String = format!("/error?code={}&message={}", code, message);
    let redirect: Redirect = Redirect::to(&to);
    let mut response: Response<Body> = redirect.into_response();
    let h_map: &mut HeaderMap = response.headers_mut();
    h_map.insert(
        "HX-Redirect",
        HeaderValue::from_str(&to)
            .unwrap()
    );
    response
}

/// # Extract user.
/// The `extract_user` function attempts to parse the user ID from cookies
/// and fetch the corresponding user from the database.
/// If any step fails, it returns a default value.
/// - Note: However, this approach can lead to potential issues if parsing or fetching operations fail unexpectedly.

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

    let id:i64 = match session {
        Some(session) => session
            .value()
            .parse::<i64>()
            .map_err(|_| StatusCode::BAD_REQUEST)?,
        None => -1,
        };

    // Get the user
    match sqlx::query_as!(
            UserDTO,
            "SELECT users.*, settings.openai_api_key
            FROM users
                LEFT JOIN settings
                ON settings.user_id=users.id
                WHERE users.id = $1",
            id
        )
    .fetch_optional(&*state.pool)
    .await
    {
        Ok(current_user) => {
            // insert the current user into a request extension, so the handler can
            // extract it, and make sure `user` is not used after this point.
            req.extensions_mut().insert(Some(current_user));
            //next.run(req).await
            Ok(next.run(req).await)
        }
        Err(SqlError::RowNotFound) => {
            // Handle specific error case for missing user
            req.extensions_mut().insert(
                None::<UserDTO>
            );
            // insert the current user into a request extension, so the handler can
            // extract it, and make sure `user` is not used after this point
            //next.run(req).await
            Ok(next.run(req).await)
                },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// # Authentication Middleware (`auth`)
/// This middleware checks if the user is logged in and redirects to an error page if not.
pub async fn auth<B>(
    Extension(current_user): Extension<Option<UserDTO>>,
    request: Request<Body>,
    next: Next,
) -> Response
where
    B: Send + 'static,
{   // не очень пойму зачем это
    let to = format!("/error?code={}&message={}", "401", "Log in");
    let r = Redirect::to(&to);
    let mut r = r.into_response();
    let h = r.headers_mut();
    h.insert("HX-Redirect", HeaderValue::from_str(&to).unwrap());
    // до этого момента, похоже обработчика нет
    match current_user {
        Some(_) => next.run(request).await,
        None => error_response(401, "You need to log in to view this page"),
    }
}

///
pub async fn valid_openai_api_key<B>(
    Extension(current_user): Extension<Option<UserDTO>>,
    req: Request<Body>,
    next: Next,
) -> Response
where
    B: Send + 'static,
{
    let key: &str = user_dto::get_open_ai_api_key(&current_user);


    if key.is_empty() {
            return error_response(
                403,
                "You API key is not set or invalid. Go to Settings."
            );
        }

    let client = reqwest::Client::new();
    match client
        // [ ]: TODO - get the env variant
        .get("https://api.openai.com/v1/engines")
        .bearer_auth(&key)
        .send()
        .await
    {
        Ok(res) => {
            if res.status().is_success() {
                next.run(req).await
            } else {
                error_response(
                    403,
                    "You API key is not set or invalid. Go to Settings."
                )
            }
        },
        Err(_) => error_response(403, "You API key is not set or invalid. Go to Settings"),
    }
}

///  — Ensure that custom errors are rendered properly.
pub async fn handle_error<B>(
    Extension(current_user): Extension<Option<UserDTO>>,
    State(state): State<Arc<AppState>>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode>
where
    B: Send + 'static,
{
    let response: Response<Body> = next.run(request).await;

    if response.status().as_u16() >= 400 {
        let status_code = response.status().as_u16();
        let status_text = response.status().as_str()
            .to_string();

        let mut context: Context = Context::new();
        context.insert("status_code", &status_code);
        context.insert("status_text", &status_text);

        let error_template = state
            .tera
            .render("views/error.html", &context)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut context = Context::new();
        context.insert("view", &error_template);
        context.insert("current_user", &current_user);
        context.insert("with_footer", &true);
        let rendered = state.tera.render(
            "views/main.html",
            &context
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Html(rendered).into_response())
    } else {
        Ok(response)
    }
}
