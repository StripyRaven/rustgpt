// -----------------------------------------------------------------------
// Locals
// -----------------------------------------------------------------------
//crate super::auth;
use super::auth::{form_signup, login, login_form, logout, signup};
use super::blog::{blog, blog_by_slug};
use super::chat::{chat, chat_add_message, chat_by_id, delete_chat, generate_chat, new_chat};
use super::error::error;
use super::home::home_app;
use super::settings::{settings, settings_openai_api_key};
use crate::model::app_state::{AppStateProject, SharedAppState};
use crate::model::project_error::ErrorMessage;
use crate::project_middleware::{auth, handle_error, valid_openai_api_key};

use axum::error_handling;
// -----------------------------------------------------------------------
// EXTERNAL
// -----------------------------------------------------------------------
use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use std::sync::Arc;
// use tracing::{debug, info};
// auth layer meets here some times

/**
 * # App router
 * Router for all app
 *  - /
 *  - /home
 *  - /blog
 *  - /blog/:slug
 *  - /chat
 *  - /chat/:id
 *  - /chat/:id/message/add
 *  - /chat/:id/generate
 *  - /login
 *  - /signup
 *  - /logout
 *  - /settings
 * - /error
 *  - /error/:code
 * - /error/:code/:message
 * - /error/:code/:message/:description
 *  * [Axum](https://www.shuttle.dev/blog/2023/12/06/using-axum-rust)
 */
//#[axum::debug_handler]
pub fn app_router(state: SharedAppState) -> Router {
    tracing::info!(
        "
    APP_ROUTER
    START"
    );
    // set router for app
    let chat_router = Router::new()
        .route("/", get(chat).post(new_chat))
        .route("/:id", get(chat_by_id).delete(delete_chat))
        .route("/:id/message/add", post(chat_add_message))
        .route("/:id/generate", get(generate_chat))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            valid_openai_api_key,
        ))
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth)); // auth 1

    tracing::info!(
        "
    APP_ROUTER
    CHAT ROUTER INIT DONE
    START SETTINGGS ROUTER"
    );

    // set router for settings
    let settings_router = Router::new()
        .route("/", get(settings).post(settings_openai_api_key))
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth)); //auth 2

    tracing::info!(
        "
    APP_ROUTER
    SETTINGS ROUTER DONE
    START MAIN ROUTER"
    );

    Router::new()
        .route("/", get(home_app))
        .route("/error", get(error))
        .route("/login", get(login).post(login_form))
        .route("/signup", get(signup).post(form_signup))
        .route("/logout", get(logout))
        .route("/blog", get(blog))
        .route("/blog/:slug", get(blog_by_slug))
        .nest("/chat", chat_router)
        .nest("/settings", settings_router)
        //.layer(axum::middleware::from_fn_with_state(
        //  state.clone(),
        //handle_error,
        //))
        .with_state(state.clone())
}
