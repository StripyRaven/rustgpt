// LOCAL
use crate::model::{app_state::AppStateProject, user::User};

use axum::{
    extract::{Extension, State},
    response::Html,
};

use tera::Context;

use std::sync::Arc;

#[axum::debug_handler]
pub async fn home_app(
    State(state): State<Arc<AppStateProject>>,
    Extension(current_user): Extension<Option<User>>,
) -> Html<String> {
    let mut context = Context::new();
    context.insert("name", "World");

    let home = state.tera.render("views/home.html", &context).unwrap();

    let mut context = Context::new();
    context.insert("view", &home);
    context.insert("current_user", &current_user);
    context.insert("with_footer", &true);
    let rendered = state.tera.render("views/main.html", &context).unwrap();

    Html(rendered)
}
