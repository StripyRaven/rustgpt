// LOCAL
mod model;
mod project_middleware;
use model::{app_state, repository::ChatRepository};
mod router;
use router::app::app_router::app_router;

// EXTERNAL
mod ai_layer;
use axum::{
    //extract::RequestParts,
    http::{
        //HeaderMap,
        StatusCode,
        //Request
    },
    //response::{
    //IntoResponse,
    //Response,
    //},
    Router,
    //routing::get,
    //extract::State,
};

use dotenv;

// use serde::Serialize;

use crate::model::app_state::AppState;
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    //Pool, Sqlite,
};
use std::{net::SocketAddr, path::Path, sync::Arc, time::Duration};
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// ----------------------------------------------------------------------------
// MAIN
// ----------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path = dotenv::var("DATABASE_PATH").unwrap(); // move to get db creds

    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true);

    // setup connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await
        .expect("can't connect to database");

    // Create a new instance of `Migrator` pointing to the migrations' folder.
    let migrator = Migrator::new(Path::new(dotenv::var("MIGRATIONS_PATH").unwrap().as_str()))
        .await
        .unwrap();

    // Run the migrations.
    migrator.run(&pool).await.unwrap();

    let pool = Arc::new(pool);

    let chat_repo = ChatRepository { pool: pool.clone() };

    let static_files = ServeDir::new("assets");

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let state = app_state::AppState {
        pool,
        tera,
        chat_repo,
    };

    let shared_app_state = Arc::new(state);

    // build our application with some routes
    let var_handel_err = axum::middleware::from_fn_with_state(
        shared_app_state.clone(),
        project_middleware::handle_error::<()>,
    );

    let app = Router::new()
        // .route(
        //     "/",
        //     get(using_connection_pool_extractor).post(using_connection_pool_extractor),
        // )
        // Use `merge` to combine routers
        .nest_service("/assets", static_files)
        .with_state(shared_app_state.clone())
        .nest("/", app_router::<()>(shared_app_state.clone()))
        .layer(var_handel_err)
        .layer(axum::middleware::from_fn_with_state(
            shared_app_state.clone(),
            project_middleware::extract_user::<()>,
        ))
        .layer(CookieManagerLayer::new());

    // run it with hyper
    let soket_addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::debug!("listening on {}", soket_addr);

    // https://crates.io/crates/axum-server
    axum_server::bind(soket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
// TODO move to error
// there is no refs in project
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/*
enum TracingError {
    B,
}
*/
