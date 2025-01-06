#![allow(dead_code)]

use super::repository::ChatRepository;
use sqlx::{Pool, Sqlite};
use std::sync::{Arc, Mutex};
// use tera::Tera;

#[derive(Clone, Debug)]
// TODO: use add UsurDTO to AppStateProject
pub struct AppStateProject {
    pub pool: Arc<Pool<Sqlite>>,
    pub tera_templates: tera::Tera,
    pub chat_repo: ChatRepository,
}

/// # SharedAppState
/// local type defiinition
/// ## To get pool example
/// ```rs
///  fn some_fn (state: SharedAppState) -> Result<(), sqlx::Error> {
/// let app_state: std::sync::Mutex<AppStateProject> = Arc::try_unwrap(state).unwrap();
/// let state: &AppStateProject = &*app_state.lock().unwrap();
/// let pool: &sqlx::Pool<sqlx::Sqlite> = &*state.pool;
///  # ...... some code
/// .fetch_optional(pool)
/// .await
/// # ...... some code
/// }
///```
pub type SharedAppState = Arc<Mutex<AppStateProject>>;

// pub struct SharedAppStateProject<SharedAppState> {
//     pub pool: Arc<Pool<Sqlite>>,
//     pub tera_templates: tera::Tera,
//     pub chat_repo: ChatRepository,
// }

// impl SharedAppStateProject<SharedAppState> {
//     pub fn new(self) -> self<SharedAppState> {
//         SharedAppStateProject::<SharedAppState> {
//             pool: aps.pool,
//             tera_templates: aps.tera_templates,
//             chat_repo: aps.chat_repo,
//         }
//     }

//     pub fn get_template(&self) -> &tera::Tera {
//         &self.tera_templates
//     }
// }
