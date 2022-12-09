use diesel::prelude::*;
use crate::schema::users;

#[derive(Queryable,Debug)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name= users)]
pub struct NewUser<'a> {
    // pub name: String,
    pub user_name: &'a str,
    pub email: &'a str,
}