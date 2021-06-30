use crate::db;
use crate::error_handler::CustomError;
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String
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
    pub phone_number: String
}

impl Users {
    pub fn login( username: String, password: String) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = users::table.filter( users::username.eq(username)
        ).filter( users::password.eq(password)
        ).first(&conn)?;
        Ok(user)
    }
}
impl Users {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let users = users::table.load::<Users>(&conn)?;
        Ok(users)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = users::table.filter(users::id.eq(id)).first(&conn)?;
        Ok(user)
    }

    pub fn create(user: User) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn update(id: i32, user: User) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
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
            phone_number: user.phone_number
        }
    }
}
