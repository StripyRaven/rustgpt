///[Utc](https://docs.rs/chrono/latest/chrono/struct.Utc.html)
/// `NaiveDateTime` is a type provided by the `chrono`
/// crate for representing a date and time without any reference
/// to a specific timezone.
/// It contains information about the year, month, day, hour, minute, second, and microsecond.
use chrono::{
    // DateTime,
    // Local,
    // Utc,
    //TimeZone,
    // NaiveDate,
    NaiveDateTime,
    //Utc,
};
use serde::{Deserialize, Serialize};

///
/// # UserDTO
/// for db mapping on auth and transfering
///
/// - [Local](https://docs.rs/chrono/latest/chrono/offset/struct.Local.html)
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserDTO {
    pub id: i64,
    pub email: String,
    // Note: Storing plain-text passwords is not recommended. Use hashed passwords instead.
    pub password: String,
    pub created_at: NaiveDateTime,
    // pub openai_api_key: String,
    pub openai_api_key: Option<String>,
}

pub fn get_open_ai_api_key<'a>(u: &Option<UserDTO>) ->&str{
    let key: &str = u.as_ref()
        .and_then(|user| user.openai_api_key.as_deref())
        .unwrap_or(""); // if Some - ok if None = "" (empty)
    key
}
