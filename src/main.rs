#![allow(unused_imports)]
// ---------------------------------------------------------------------------
// LOCAL
// ---------------------------------------------------------------------------
mod model;
mod project_middleware;
use model::{app_state, repository::ChatRepository};
mod router;
use router::app::app_router::app_router;
mod ai_layer;
// ---------------------------------------------------------------------------
// EXTERNAL
// ---------------------------------------------------------------------------
use axum::Router;
use dotenv;
// use serde::Serialize;
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    //Pool, Sqlite,
};
use std::{
    net::SocketAddr,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{debug, error, info, Level};
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt,
    prelude::*,
    util::SubscriberInitExt,
};
// ----------------------------------------------------------------------------
// MAIN
// ----------------------------------------------------------------------------

/// - [Tracing](https://www.shuttle.dev/blog/2024/01/09/getting-started-tracing-rust)
/// - [Tokyo & Tracing](https://tokio.rs/tokio/topics/tracing)
/// - [request](https://docs.rs/http/1.2.0/src/http/request.rs.html#158-161)
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // ---------------------------------------------------------
    // "TRACING SUBSCRIBER INIT"
    //----------------------------------------------------------
    if dotenv::var("LOG_FORMAT").is_ok_and(|log| log == "JSON") {
        //tracing_subscriber::fmt()
        tracing_subscriber::fmt::format().json();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .event_format(
                tracing_subscriber::fmt::format()
                    //.without_time(false)
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_source_location(true)
                    //.pretty(),
                    .compact(),
            )
            .init();
    }
    //---------------------------------------------------------

    info!("Application starting");

    let db_path = dotenv::var("DATABASE_PATH").expect("DATABASE_PATH must be set");
    let migrations_path = dotenv::var("MIGRATIONS_PATH").expect("MIGRATION_PATH must be set");

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
    let migrator = Migrator::new(Path::new(&migrations_path)).await.unwrap();

    // Run the migrations.
    migrator.run(&pool).await.unwrap();

    info!("db init done");

    let pool = Arc::new(pool);

    let chat_repo = ChatRepository { pool: pool.clone() };

    let static_files = ServeDir::new("assets");

    // create tera instance
    // from files stored in 'templates' folder
    let tera_templates = match Tera::new("templates/**/*") {
        Ok(_t) => _t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tracing::info!(
        "
        SET TEPMPLATES
        PAGES: {},
        {:#?}",
        tera_templates.templates.len(),
        tera_templates.templates.keys()
    );

    // create app state
    let state = app_state::AppStateProject {
        pool,
        tera_templates,
        chat_repo,
    };

    // create shared state
    // this is shared between all requests
    // let state = Arc::new(state.clone());

    let _s = Arc::new(Mutex::new(state));

    //TODO: const to be used or enum as routes registry
    let main_template = "/";
    let asset_template = "/assets";

    tracing::info!(
        "
    STARTING MAIN ROUTER"
    );

    // create router
    // this is the main router
    // it contains all the routes
    let app = Router::new()
        /* .route(
            "/",
            get(using_connection_pool_extractor).post(using_connection_pool_extractor),
            )
            Use `merge` to combine routers
        */
        .nest_service(asset_template, static_files)
        // change on _s to use the state
        .with_state(_s.clone())
        // app_router
        .nest(main_template, app_router(_s.clone()))
        // handle err
        .layer(axum::middleware::from_fn_with_state(
            _s.clone(),
            project_middleware::handle_error,
        ))
        // extract user
        .layer(axum::middleware::from_fn_with_state(
            _s.clone(),
            project_middleware::extract_user,
        ))
        .layer(CookieManagerLayer::new());

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 3001));

    tracing::info!("LISTENING ON {}", socket_addr);

    // **Identify the Process Using the Port**:
    //   You can use tools like `lsof` (on Unix-like systems) or `netstat`/`ss` (also on Unix-like systems) to find out which process is using the port.
    //```sh
    // lsof -i :3000
    // ```
    // `Err` value: Os { code: 48, kind: AddrInUse, message: "Address already in use" }
    match axum_server::bind(socket_addr)
        .serve(app.into_make_service())
        .await
    {
        Ok(_) => tracing::info!("OK - Server started on {}", socket_addr),
        Err(e) => tracing::error!("Err - Failed to start server: {}", e),
    };
    /* TODO
        - check and restart service
        - start tailwindcss
        - open browser
    */

    tracing::error!("tracing: An error occurred");
}
