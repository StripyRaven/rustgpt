use super::repository::ChatRepository;
use std::sync::Arc;
use sqlx::{Pool, Sqlite};
// use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool<Sqlite>>,
    pub tera: tera::Tera,
    pub chat_repo: ChatRepository,
}
