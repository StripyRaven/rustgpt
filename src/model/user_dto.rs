#![allow(dead_code)]
//-----------------------------------------------------------------------------
// Local
//-----------------------------------------------------------------------------
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

/**
    ## struct UserDTO
        model for db mapping on auth and transfering
        - [Local](https://docs.rs/chrono/latest/chrono/offset/struct.Local.html)
*/
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

///////////////////////////////////////////////////////////////////////////////

impl UserDTO {
    /**
        ## GET OPENAI API KEY
        getting `UserDTO` and extract from `Option<<Option<String>>
        - OK -> Key
        - Err -> no `user` og no `key`
        - Note: Result as `String`
        - note: to be matchable with db types
        - TODO collect types as ENUM enum {CHAT, MIDDLEWARE, DB .... etc}
        - [Sqlx::Sqlite datatypes](https://docs.rs/sqlx/latest/sqlx/sqlite/types/index.html)
    */
    pub fn get_open_ai_api_key(&self) -> Result<String, String> {
        match &self.openai_api_key {
            Some(key) => Ok(key.to_string()),
            None => Err("400, key wrong or not exist".to_string()),
        }
    }

    /**
        ## GET OPENAI API KEY FROM OPTION
        This method is intended to handle the case where you have an `Option<UserDTO>`
        and need to extract the open AI API key.
        ## Examples
        ```rs
        struct UserDTO {
            openai_api_key: Option<String>,
        }

        fn main() {
            let user = UserDTO {
                openai_api_key: Some("your_api_key".to_string())
            };

            match user.get_open_ai_api_key() {
                Ok(key) => println!("OpenAI API Key: {}", key),
                Err(e) => println!("Error: {}", e),
            }

            let opt_user: Option<&UserDTO> = Some(&user);

            match UserDTO::get_open_ai_api_key_fm_option(opt_user) {
                Ok(key) => println!("OpenAI API Key from Option: {}", key),
                Err(e) => println!("Error: {}", e),
            }
        }
        ```
    */
    pub fn get_open_ai_api_key_fm_option(opt: Option<&Self>) -> Result<String, String> {
        match opt {
            Some(user) => user.get_open_ai_api_key(),
            None => Err("400, user wrong or not exist".to_string()),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
