// LOCAL
use crate::model::{app_state::AppStateProject, user_dto::UserDTO};

// EXTERNAL
use axum::{
    extract::{Extension, State},
    response::Html,
};

use tera::Context;

use std::sync::Arc;

/**
# fn home_app
Rendering home page
## params
- state
- current_user
## return
- Html<String>
 */
#[axum::debug_handler]
pub async fn home_app(
    State(state): State<Arc<AppStateProject>>,
    Extension(current_user): Extension<Option<UserDTO>>,
) -> Html<String> {
    tracing::info!(
        "
        ENTER HOME
        CURRENT_USER: {:?}",
        &current_user
    );

    let mut context = Context::new();

    context.insert("name", "World");

    let home = state
        .tera_templates
        .render("views/home.html", &context)
        .unwrap();

    let mut context = Context::new();
    context.insert("view", &home);
    context.insert("current_user", &current_user);
    context.insert("with_footer", &true);

    let rendered = state
        .tera_templates
        .render("views/main.html", &context)
        .unwrap();

    Html(rendered)
}

/*
<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="/assets/output.css" /link>
    <script src="https://unpkg.com/htmx.org@1.9.6"
        integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
        crossorigin="anonymous"></script>
    <script src="https://unpkg.com/htmx.org/dist/ext/sse.js"></script>
</head>

<body>
    {% include "components/header.html" %}
    <main class="pt-[60px]">
        {{ view | safe }}
    </main>

    {% if with_footer %}
    {% include "components/footer.html" %}
    {% endif %}
</body>
 */
