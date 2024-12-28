// LOCAL
use crate::model::{app_state::AppStateProject, project_error::ErrorMessage, user_dto::UserDTO};

use axum::{
    extract::{Extension, Query, State},
    response::Html,
};
//use serde::Deserialize;
use std::sync::Arc;
use tera::Context;

//#[axum::debug_handler]
pub async fn error(
    Query(params): Query<ErrorMessage>,
    State(state): State<Arc<AppStateProject>>,
    Extension(current_user): Extension<Option<UserDTO>>,
) -> Html<String> {
    let err_tmp: &str = "views/error.html";

    let mut context = Context::new();
    context.insert("status_code", &params.code);
    context.insert("status_text", &params.message);

    // Ensure that the error handling is done properly
    if let Err(err) = state.tera.render(err_tmp, &context) {
        return Html(format!("Error rendering template: {}", err));
    }

    let rendered_error_template = state.tera.render(err_tmp, &context).unwrap();

    let mut context = Context::new();
    context.insert("view", &rendered_error_template);
    context.insert("current_user", &current_user);
    context.insert("with_footer", &true);

    if let Err(err) = state.tera.render("views/main.html", &context) {
        return Html(format!("Error rendering template: {}", err));
    }
    // returns
    let rendered = state.tera.render("views/main.html", &context).unwrap(); // as var

    Html(rendered)
}
