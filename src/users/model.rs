use crate::db;

use crate::schema::users;
use diesel::prelude::*;
use note_utils::my_serde::{Deserialize, Serialize};
use diesel::sql_query;
use diesel::sql_types::Integer;
use note_utils::MyError;


#[derive(Debug,Serialize, Deserialize, AsChangeset, Insertable)]
#[serde(crate = "note_utils::my_serde")]
#[table_name = "users"]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize, Queryable, Insertable,QueryableByName)]
#[serde(crate = "note_utils::my_serde")]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String
}


impl User{

    pub fn find(conn: &PgConnection, id: i32) -> Result<Vec<User>, MyError> {

        let users = sql_query("SELECT * FROM users  WHERE id = $1 ");

        let users =   users.bind::<Integer, _>(1)
            .load(conn)?;

        Ok(users)
    }

}

impl User {
    fn from(user: User) -> User {
        User {
            id: 0,
            username: user.username,
            password: user.password,
            first_name: user.first_name,
            last_name: user.last_name,
            phone_number: user.phone_number
        }
    }
}
