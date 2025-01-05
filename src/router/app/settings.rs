// LOCAL
use crate::model::{app_state::AppStateProject, user_dto::UserDTO};

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::{Html, Redirect},
    Form,
};

use serde::Deserialize;
use std::sync::Arc;

use tera::Context;
use tracing::{error, info};

#[derive(Deserialize, Debug)]
pub struct OpenAiAPIKey {
    api_key: String,
}

#[axum::debug_handler]
pub async fn settings_openai_api_key(
    State(state): State<Arc<AppStateProject>>,
    Extension(current_user): Extension<Option<UserDTO>>, //Correct user
    Form(set_openai_api_key): Form<OpenAiAPIKey>,
) -> Result<Redirect, StatusCode> {
    tracing::info!(
        "
    SETTING OPEN AI KEY"
    );

    let id = current_user.unwrap().id; // did it true?

    tracing::info!(
        "
    USER.ID: {}
    INSERT KEY TO BD",
        &id
    );

    sqlx::query!(
        r#"INSERT INTO settings (user_id, openai_api_key) VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE SET openai_api_key = $2"#,
        id,
        set_openai_api_key.api_key
        //set_openai_api_key.api_key
    ).execute(&*state.pool).await.unwrap();

    Ok(Redirect::to("/settings"))
}

#[axum::debug_handler]
pub async fn settings(
    State(state): State<Arc<AppStateProject>>,
    Extension(current_user): Extension<Option<UserDTO>>,
) -> Result<Html<String>, StatusCode> {
    tracing::info!("ENTER SETTINGS");
    let key = current_user.as_ref().unwrap().openai_api_key.as_ref();

    // set the contexxt of the template
    //
    let mut context = Context::new();
    context.insert("openai_api_key", &key);

    // set the current user into a request extension, so the handler can
    // access it
    let settings = state
        .tera_templates
        .render("views/settings.html", &context)
        .unwrap();

    let mut context = Context::new();
    context.insert("view", &settings);
    context.insert("current_user", &current_user);
    context.insert("with_footer", &true);

    let rendered = state
        .tera_templates
        .render("views/main.html", &context)
        .unwrap();

    Ok(Html(rendered))
}
