use crate::db;
use crate::error_handler::{MyError, ApiError};
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::debug_query;
use diesel::expression::dsl::sql;
use self::chrono::{NaiveDate, Local, NaiveDateTime};
use diesel::sql_types::{Date, Timestamp};
use std::str::FromStr;
use diesel::pg::expression::extensions::IntervalDsl;
use diesel::expression::functions::date_and_time::now;

extern crate chrono;

#[derive(Debug,Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub created_at: Option<NaiveDateTime>
}

#[derive(Debug,Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub created_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[table_name = "users"]
pub struct UserUpdate {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationFilterMessage {
    pub text: String,
    pub status: Option<i16>,
    pub current_page: Option<i64>,
    pub record_per_page: Option<i64>,
    pub sort_by: Option<String>,
}


impl Users {
    pub fn login( username: String, password: String) -> Result<Self, MyError> {
        let conn = db::connection()?;
        let user = users::table.filter( users::username.eq(username)
        ).filter( users::password.eq(password)
        ).first(&conn)?;
        Ok(user)
    }
}
impl Users {
    pub fn find_all() -> Result<Vec<Self>, MyError> {
        let conn = db::connection()?;
        let users = users::table.load::<Users>(&conn)?;
        Ok(users)
    }

    pub fn find(id: i32) -> Result<Self, MyError> {
        let conn = db::connection()?;
        let user = users::table.filter(users::id.eq(id)).first(&conn)?;
        Ok(user)
    }

    pub fn create(user: User) -> Result<Self, MyError> {
        let conn = db::connection()?;
        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn update(id: i32, user: UserUpdate) -> Result<Self, MyError> {
        let conn = db::connection()?;
        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn delete(id: i32) -> Result<usize, MyError> {
        let conn = db::connection()?;
        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl User {
    fn from(user: User) -> User {
        User {
            username: user.username,
            password: user.password,
            first_name: user.first_name,
            last_name: user.last_name,
            phone_number: user.phone_number,
            created_at: Some(Local::now().naive_local())
        }
    }
}

impl UserUpdate {
    fn from(user: UserUpdate) -> UserUpdate {
        UserUpdate {
            first_name: user.first_name,
            last_name: user.last_name,
            phone_number: user.phone_number
        }
    }
}

