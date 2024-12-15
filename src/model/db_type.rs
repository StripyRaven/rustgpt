// use sqlx::{
    // database,
    // postgres::PgPoolOptions,
    // mysql::MySqlPoolOptions,
    // sqlite::SqlitePoolOptions
// };

// use std::env;

pub enum Database {
    Postgres,
    MySql,
    Sqlite,
}

/*
impl sqlx::database for Database {
    fn name(&self) -> &'static str {
        match self {
            Database::Postgres => "postgres",
            Database::MySql => "mysql",
            Database::Sqlite => "sqlite",
        }
    }

    fn supports_uuids(&self) -> bool {
        matches!(self, Database::Postgres | Database::Sqlite)
    }
}
*/

const POSTGRES_URL: &str = "postgres://...";
const MYSQL_URL: &str = "mysql://...";
const SQLITE_URL: &str = "sqlite://...";

impl Database {
    pub fn get_connection_string(&self) -> &'static str {
        match self {
            Database::Postgres => POSTGRES_URL,
            Database::MySql => MYSQL_URL,
            Database::Sqlite => SQLITE_URL,
        }
    }

    // ... rest of the implementation ...
}
