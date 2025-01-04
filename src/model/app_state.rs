#![allow(dead_code)]

use super::repository::ChatRepository;
use sqlx::{Pool, Sqlite};
use std::sync::{Arc, Mutex};
// use tera::Tera;

#[derive(Clone)]
pub struct AppStateProject {
    pub pool: Arc<Pool<Sqlite>>,
    pub tera_templates: tera::Tera,
    pub chat_repo: ChatRepository,
}

pub type SharedAppState = Arc<Mutex<AppStateProject>>;
