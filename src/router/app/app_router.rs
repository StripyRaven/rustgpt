/////////////////////////////////////////////////////////////////////
// Locals
/////////////////////////////////////////////////////////////////////
//crate super::auth;
use super::auth::{form_signup, login, login_form, logout, signup};
use super::blog::{blog, blog_by_slug};
use super::chat::{chat, chat_add_message, chat_by_id, delete_chat, generate_chat, new_chat};
use super::error::error;
use super::home::home_app;
use super::settings::{settings, settings_openai_api_key};
use crate::model::app_state::AppStateProject;
use crate::project_middleware::{auth, valid_openai_api_key};

/////////////////////////////////////////////////////////////////////
// EXTERNAL
/////////////////////////////////////////////////////////////////////
use axum::{
    body::Body as body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};

use std::sync::Arc;
use tracing::{error, info};

pub fn app_router<B>(state: Arc<AppStateProject>) -> Router
where
    B: Send + 'static,
{
    let chat_router = Router::new()
        .route("/", get(chat).post(new_chat))
        .route("/:id", get(chat_by_id).delete(delete_chat))
        .route("/:id/message/add", post(chat_add_message))
        .route("/:id/generate", get(generate_chat))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn(valid_openai_api_key::<B>))
        .layer(axum::middleware::from_fn(auth::<B>));

    let settings_router = Router::new()
        .route("/", get(settings).post(settings_openai_api_key))
        .layer(axum::middleware::from_fn(auth::<B>));

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
        .layer(axum::middleware::from_fn(catch_errors::<B>)) // Apply middleware directly
        .with_state(state.clone())
}

pub async fn catch_errors<B>(req: Request<body>, next: Next) -> impl IntoResponse {
    let response = next.run(req).await;

    if !response.status().is_success() {
        error!("Error occurred: {:?}", response);
    }

    response
}
