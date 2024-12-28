/// LOCAL
use crate::model::{app_state::AppStateProject, project_error::ErrorMessage, user_dto::UserDTO};

use axum::{
    body::Body,
    //async_trait,
    extract::State,
    http::{
        //HeaderMap,
        HeaderValue,
        Request,
        StatusCode,
    },
    middleware::Next,
    response::{
        Extension,
        //User,
        // client::Client,
        Html,
        IntoResponse,
        Redirect,
        Response,
    },
};
//use::future::Future;
use tracing;

use sqlx::Error as SqlError;
use std::sync::Arc; // Sqlx use Arq, check via tree

use tera::Context;
use tower_cookies::Cookies;

/// # Extract user.
/// The `extract_user` function attempts to parse the user ID from cookies
/// and fetch the corresponding user from the database.
/// If any step fails, it returns a default value.
/// - Note: However, this approach can lead to potential issues if parsing or fetching operations fail unexpectedly.

// TODO split into two part SQL requst and assertion
pub async fn extract_user<B>(
    State(state): State<Arc<AppStateProject>>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode>
where
    B: Send + 'static,
{
    let session = cookies.get("rust-AI-session");

    let id: i64 = match session {
        Some(session) => session
            .value()
            .parse::<i64>()
            .map_err(|_| StatusCode::BAD_REQUEST)?,
        None => -1,
    };

    // Get the user - turn to function
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
        Ok(Some(current_user)) => {
            // insert the current user into a request extension, so the handler can
            // extract it, and make sure `user` is not used after this point.
            req.extensions_mut().insert(current_user);
            //next.run(req).await
            Ok(next.run(req).await)
        }
        // Ok(None) => TODO to be None Option<UserDTO>
        Ok(None) | Err(SqlError::RowNotFound) => {
            // Handle specific error case for missing user
            req.extensions_mut().insert(None::<UserDTO>);
            // insert the current user into a request extension, so the handler can
            // extract it, and make sure `user` is not used after this point
            //next.run(req).await
            Ok(next.run(req).await)
        }
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
{
    // не очень пойму зачем это
    let to = format!("/error?code={}&message={}", "401", "Log in");
    let r = Redirect::to(&to);
    let mut r = r.into_response();
    let h = r.headers_mut();
    h.insert("HX-Redirect", HeaderValue::from_str(&to).unwrap());
    // до этого момента, похоже обработчика нет
    match current_user {
        Some(_) => next.run(request).await,
        None => ErrorMessage::new(
            StatusCode::UNAUTHORIZED.as_u16(),
            "You need to log in to view this page",
        )
        .into_response(),
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
    //let cud = current_user.clone().unwrap();
    if let Some(user_dto) = current_user {
        let key = user_dto
            .get_open_ai_api_key()
            .unwrap_or("400: key unexist".to_string());

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
                    ErrorMessage::new(
                        StatusCode::FORBIDDEN.as_u16(),
                        "You API key is not set or invalid. Go to Settings.",
                    )
                    .into_response()
                }
            }
            Err(_) => ErrorMessage::new(
                StatusCode::FORBIDDEN.as_u16(),
                "You API key is not set or invalid. Go to Settings",
            )
            .into_response(),
        }
    } else {
        //let key = "400: key unexist";
        ErrorMessage::new(400, "You API key is not set or invalid. Go to Settings").into_response()
    }
}

///  — Ensure that custom errors are rendered properly.
/// - [http statuses](https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)
// TO
// pub async fn handle_error<B>(req: Request<B>) -> Result<impl IntoResponse, StatusCode>
// where
//   B: Send + 'static,
pub async fn handle_error<B>(
    Extension(current_user): Extension<Option<UserDTO>>,
    State(state): State<Arc<AppStateProject>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode>
where
    B: Send + 'static,
{
    let response = next.run(request).await;

    let status_code = response.status().as_u16();
    let status_text = response.status().as_str().to_string();

    match status_code {
        _ if status_code >= 400 => {
            let mut context: Context = Context::new();
            context.insert("status_code", &status_code);
            context.insert("status_text", &status_text);

            // Handle erroe
            let error_template = state.tera.render("views/error.html", &context).unwrap();

            let mut context = Context::new();
            context.insert("view", &error_template);
            context.insert("current_user", &current_user);
            context.insert("with_footer", &true);

            // Yandle error
            let rendered = state.tera.render("views/main.html", &context).unwrap();

            // for debug via type imlay hints
            // Response<Body>
            let rsp_tmp = Html(rendered).into_response();
            // Result<Response, StatusCode>
            Ok(rsp_tmp)
        }
        _ => {
            // response.status().as_u16() < 400 {
            Ok(response)
        }
    }
}
