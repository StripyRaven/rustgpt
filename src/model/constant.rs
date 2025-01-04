#![allow(dead_code)]
//* value for undefined id of user
pub const ID_USER_NONE: i64 = 0;
// DB
pub const POSTGRES_URL: &str = "postgres://...";
pub const MYSQL_URL: &str = "mysql://...";
pub const SQLITE_URL: &str = "sqlite://...";
// SESSION
pub const SESSION_NAME: &str = "rust-ai-session";
pub const SESSION_KEY: &str = "rust-ai-key";
pub const SESSION_COOKIE_NAME: &str = "rust-ai-cookie";

pub const SESSION_SECRET: &str = "rust-ai-secret";
pub const SESSION_TIMEOUT: usize = 1000;
// TEMPLATE
pub const ERR_PAGE_TEMPLATE: &str = "views/error.html";
pub const MAIN_TEMPLATE: &str = "views/main.html";
pub const ASSET_TEMPLATE: &str = "/assets";
