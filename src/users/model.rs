use std::fmt;
use std::str::FromStr;

use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize, de};
use uuid::Uuid;

use crate::db;
use crate::schema::users;
use note_utils::MyError;

// use crate::vendor::google::GoogleUserInfo;
// use crate::vendor::facebook::FacebookUserInfo;

#[derive(Debug, Clone, Copy, diesel_derive_enum::DbEnum, PartialEq, Deserialize, Serialize)]
pub enum UserExternalIDP {
    Google,
    Facebook,
    // Twitter,
    Line,
    Email,
    Phone,
    System,
    Guest,
}

impl fmt::Display for UserExternalIDP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for UserExternalIDP {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str = s.to_lowercase();
        Ok(match str.as_str() {
            "google" | "google.com" => UserExternalIDP::Google,
            "facebook" | "facebook.com" => UserExternalIDP::Facebook,
            "line" => UserExternalIDP::Line,
            "email" => UserExternalIDP::Email,
            "phone" => UserExternalIDP::Phone,
            "system" => UserExternalIDP::System,
            _ => UserExternalIDP::Guest,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPayload {
    pub external_idp_id: f64,
    pub external_idp: UserExternalIDP,
}

impl fmt::Display for UserPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "external_idp_id: {}, external_idp: {}",
            self.external_idp_id,
            self.external_idp.to_string()
        )
    }
}

// this struct will use to receive user input
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct UserRequest {
    pub customer_id: i32,
    pub external_idp_id: String,
    pub external_idp: UserExternalIDP,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub sign_session: Option<String>,
    pub user_id: Option<String>,
    pub device_token: Option<String>,
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
    pub data: Option<String>,
    pub is_active: bool,
    pub is_internal: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// this struct will use to receive user input
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserRequestUpdate {
    pub display_name: Option<String>,
    pub sign_session: Option<String>,
}

// this struct will be used to represent database record
#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub customer_id: i32,
    pub external_idp_id: String,
    pub external_idp: UserExternalIDP,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub sign_session: Option<String>,
    pub user_id: Option<String>,
    pub device_token: Option<String>,
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
    pub data: Option<String>,
    pub is_active: bool,
    pub is_internal: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            customer_id: 0,
            external_idp_id: "".to_string(),
            external_idp: UserExternalIDP::Guest,
            email: None,
            display_name: None,
            sign_session: None,
            user_id: None,
            device_token: None,
            id_token: None,
            refresh_token: None,
            data: None,
            is_active: false,
            is_internal: false,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User id: {}, email: {:?}, external_idp_id: {:?}, external_idp: {}, display_name: {:?}, sign_session: {:?}", self.id, self.email, self.external_idp_id, self.external_idp, self.display_name, self.sign_session)
    }
}

// implementation of Actix Responder for User struct so we can return User from action handler
// impl Responder for Users {
//     type Error = Error;
//     type Future = Ready<Result<impl Responder, Error>>;
//
//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();
//
//         ready(Ok(Responder::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }

impl User {

    pub fn find_all(conn: &PgConnection) -> Result<Vec<User>, MyError> {
        let rows = users::table
            .load(conn)?;

        Ok(rows)
    }

    // find by our in house UUID
    pub fn find_by_id(conn: &PgConnection,id: &str) -> Result<User, MyError> {
        let user = users::table
            .filter(users::id.eq(id))
            .first(conn)?;
        Ok(user)
    }

}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct UserProfileUpdateMessage {
    #[serde(rename = "deviceToken")]
    pub device_token: Option<String>,
    #[serde(rename = "userDisplayName")]
    pub display_name: Option<String>,
    #[serde(rename = "userEmail")]
    pub email: Option<String>,
    // #[serde(rename = "userNiceName")]
    // pub user_nicename: Option<String>,
    // #[serde(rename = "userUrl")]
    // pub user_url: Option<String>,
}

impl From<User> for UserRequest {
    fn from(user: User) -> UserRequest {
        UserRequest {
            customer_id: user.customer_id,
            external_idp_id: user.external_idp_id,
            external_idp: user.external_idp,
            email: user.email,
            display_name: user.display_name,
            sign_session: user.sign_session,
            user_id: user.user_id,
            device_token: user.device_token,
            id_token: user.id_token,
            refresh_token: user.refresh_token,
            data: user.data,
            is_active: user.is_active,
            is_internal: user.is_internal,
            created_at: user.created_at,
            updated_at: Local::now().naive_local(),
        }
    }
}