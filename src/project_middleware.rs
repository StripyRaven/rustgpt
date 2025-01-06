/// LOCAL
use crate::model::{
    app_state::{AppStateProject, SharedAppState},
    constant::{ID_USER_NONE, SESSION_NAME},
    project_error::ErrorMessage,
    user_dto::UserDTO,
};

/// EXTERNAL
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
        // User,
        // client::Client,
        Html,
        IntoResponse,
        Redirect,
        Response,
    },
};
use futures::TryFutureExt;
use reqwest::header::IF_NONE_MATCH;
use tracing::instrument::WithSubscriber;
// -----------------------------------------------------------------------
use sqlx::Error as SqlError;
use std::{sync::Arc, thread}; // Sqlx use Arq, check via tree

use tera::Context;
use tower_cookies::Cookies;
// use tracing::info;

/// # Extract user.
/// The `extract_user` function attempts to parse the user ID from cookies
/// and fetch the corresponding user from the database.
/// If any step fails, it returns a default value.
/// - Note: However, this approach can lead to potential issues if parsing or fetching operations fail unexpectedly.

// TODO split into two part SQL requst and assertion
pub async fn extract_user(
    State(state): State<SharedAppState>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    tracing::info!(
        "
    EXTRACT USER
    ENTER MIDDLEWARE"
    );
    let session_name = cookies.get(SESSION_NAME);

    let user_id_session: i64 = match session_name {
        Some(session) => session
            .value()
            .parse::<i64>()
            .map_err(|_| StatusCode::BAD_REQUEST)?,
        None => ID_USER_NONE,
    };

    // if 0 there is no user in session cookyes - main or sign up
    if user_id_session == ID_USER_NONE {
        tracing::info!(
            "
        EXTRACT USER 
        SESSION USER.ID: {}
        DETERMINE ON STARTUP
        NONE(CURRENT_USER)
        NOT AUTHORIZED",
            &user_id_session
        );
        // TODO: go to main or sign up
        req.extensions_mut().insert(None::<UserDTO>);
        return Ok(next.run(req).await);
    }

    tracing::info!(
        "
    EXTRACT USER 
    SESSION USER.ID: {}
    EXTRACT USER DATA FM DB",
        &user_id_session
    );

    // Get the user - turn to function
    // TODO: keep session status and authed user ti avoid db request
    // NOTE: variables defined for deebuggind SQL mapping due to Mutex in use
    let app_state: std::sync::Mutex<AppStateProject> = Arc::try_unwrap(state).unwrap();
    let state: &AppStateProject = &*app_state.lock().unwrap();
    let pool: &sqlx::Pool<sqlx::Sqlite> = &*state.pool;

    // SOME(USER) and so on ....
    match sqlx::query_as!(
        UserDTO,
        r#"SELECT users.*, settings.openai_api_key
        FROM users
        LEFT JOIN settings
        ON settings.user_id=users.id
                WHERE users.id = $1"#,
        user_id_session
    )
    .fetch_optional(pool)
    .await
    {
        Ok(Some(current_user)) => {
            // insert the current user into a request extension, so the handler can
            // extract it, and make sure `user` is not used after this point.
            req.extensions_mut().insert(Some(current_user.clone()));
            tracing::info!(
                "
            EXTRACT USER 
            OK(SOME  USER) EXTRACTED, insert user to extension
            id:            {}
            email:         {}
            password       {}
            openai_api_key {:?}
            ------------------
            RESPONSE HEADERS: {:?}",
                current_user.id,
                current_user.email,
                current_user.password,
                current_user.openai_api_key,
                &req.headers().keys(),
            );
            let run = next.run(req);
            Ok(run.await)
        }
        // Ok(None) or NotFound => TODO to be None Option<UserDTO>
        Ok(None) => {
            // Handle specific error case for missing user
            req.extensions_mut().insert(None::<UserDTO>);
            tracing::info!("CURRENT USER IS NONE: {}", &user_id_session);
            // insert the current user into a request extension, so the handler can
            // extract it, and make sure `user` is not used after this point
            Ok(next.run(req).await)
        }
        Err(SqlError::RowNotFound) => {
            tracing::error!(
                "
            EXTRACT USER 
            ERROR 1 DB EXTRACTION
            TAKE A LOOK !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(SqlError::PoolTimedOut) => {
            tracing::error!(
                "
            EXTRACT USER
            ERROR 2 TRACING TAKE A LOOK !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(_) => {
            tracing::error!(
                "
            EXTRACT USER 
            ERROR 3 TRACING TAKE A LOOK !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
/**
    # Authentication Middleware (`auth`)
        This middleware checks if the user is logged in and redirects to an error page
        if not.
*/
pub async fn auth(
    Extension(current_user): Extension<Option<UserDTO>>,
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    tracing::info!("Enter AUTH MIDDLEWARE");
    // ----------------------------------------------------------------------------------
    // TODO: check next snippet wuth fn error_response
    //  if error we need to redirect to error page
    //  and insert header
    //  if ok - continue
    let to = format!("/error?code={}&message={}", "401", "Log in");
    let r = Redirect::to(&to);
    let mut r = r.into_response();
    let header_map = r.headers_mut();
    header_map.insert("HX-Redirect", HeaderValue::from_str(&to).unwrap());
    //////////////////////////////////////
    match current_user {
        Some(_user) => {
            tracing::info!("USER IS LOGGED IN");
            // continue
            next.run(req).await
        }
        _ => error_response(401, "You need to log in to view this page"),
    }
}

/**
 * # fn valid openai api key
 * This middleware checks if the user has a valid OpenAI API key.
 * If the key is invalid, it redirects to an error page.
 * If the key is valid, it continues to the next middleware.
 * If the user is not logged in, it redirects to the login page.
 * If the user has no OpenAI API key, it redirects to the settings page.
 *
 * pa
 */
pub async fn valid_openai_api_key(
    Extension(current_user): Extension<Option<UserDTO>>,
    req: Request<Body>,
    next: Next,
) -> Response<Body>
//where
    //B: Send + 'static,
{
    tracing::info!("Enter VALID OPENAI API KEY MIDDLEWARE");

    if let Some(user_dto) = current_user {
        let key = user_dto
            .get_open_ai_api_key()
            .unwrap_or("400: key unexist".to_string());

        tracing::info!("UserDTO.KEY: {}", &key);

        let client = reqwest::Client::new();
        match client
            // [ ]: TODO - get the env variant
            // [ ]: TODO set the env variant
            .get("https://api.openai.com/v1/engines") // TODO what for?
            .bearer_auth(&key)
            .send()
            .await
        {
            Ok(res) => {
                if res.status().is_success() {
                    next.run(req).await
                } else {
                    tracing::info!("ou API key is not set or invalid. Go to Settings");
                    error_response(401, "You need to log in to view this page")
                }
            }
            Err(_) => error_response(401, "You need to log in to view this page"),
        }
    } else {
        //let key = "400: key unexist";
        error_response(401, "You need to log in to view this page")
    }
}

/**
* # fn handle_error
* ## parameters:
* - `req` - Request<Body>
* - `state` - State<Arc<AppStateProject>>
* - `next` - Next
* - `Result<Response<Body>, StatusCode>`
* - [StatusCode](https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)
* - [Context](https://docs.rs/handlebars/4.3.5/handlebars/struct.Context.html)
* ## Templates
* - `views/error.html`
* - `views/main.html`
* - `views/partials/header.html`
* - `views/partials/footer.html`
*/
pub async fn handle_error(
    Extension(current_user): Extension<Option<UserDTO>>,
    State(state): State<SharedAppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode>
//where
//    B: Send + 'static,
{
    tracing::info!(
        "
            HANDLE_ERROR 
            ENTER MIDDLEWARE
            CURRENT_USER: {:?}
            SET STATUS CODE",
        &current_user
    );

    let response = next.run(req).await;

    let status_code = response.status().as_u16();
    let status_text = response.status().to_string();

    tracing::info!(
        "
            HANDLE_ERROR
            STATUS CODE:    {}
            MESSAGE:        {}
            RESP.HEADER     {:?}",
        &status_code,
        &status_text,
        &response.headers().keys()
    );

    match status_code {
        _ if status_code >= 400 => {
            let mut context: Context = Context::new();
            context.insert("status_code", &status_code);
            context.insert("status_text", &status_text);

            // Handle error
            let error_template = state
                .lock()
                .unwrap()
                .tera_templates
                // TODO: move to constants
                .render("views/error.html", &context)
                .unwrap();

            let mut context = Context::new();
            context.insert("view", &error_template);
            context.insert("current_user", &current_user);
            context.insert("with_footer", &true);

            // Handle error
            let rendered = state
                .lock()
                .unwrap()
                .tera_templates
                .render("views/main.html", &context)
                .unwrap();

            // Result<Response, StatusCode>
            // tracing::info!("GO TO {}", &rendered); // full HTML rendered page
            Ok(Html(rendered).into_response())
        }
        _ => {
            // if response.status().as_u16() < 400 {
            tracing::info!(
                "
            HANDLE_ERROR
            STATUS CODE:    {} < 400,
            HEADer.PARTS:   {:?},
            BODY.PARTS:     {:?}",
                &status_code,
                //&response.headers().get("Content-Type"),
                &response.headers().keys(),
                &response.body().with_current_subscriber(),
            );
            Ok(response)
        }
    }
}
/**
 * # fn error_response
 * ## parameters:
 * - `code` - u16
 * - `message` - &str
 * returns:
 * - Response
 * - [StatusCode](https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)
 * HX-Redirect
 * template:
 * - `views/error.html`
 */
pub fn error_response(code: u16, message: &str) -> Response {
    tracing::info!("ERROR RESPONSE");
    let to = format!("/error?code={}&message={}", code, message);
    let r = Redirect::to(&to);
    let mut r = r.into_response();
    let h = r.headers_mut();
    h.insert("HX-Redirect", HeaderValue::from_str(&to).unwrap());
    r
}
