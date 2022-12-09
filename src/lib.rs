pub mod models;
pub mod schema;
// use diesel::prelude::*;
use dotenvy::dotenv;
use models::NewUser;
use schema::users;
use std::{env, collections::HashMap};
use diesel::{mysql::MysqlConnection, Connection, RunQueryDsl};
// use crate::models::User;


pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))    
}

pub fn add_user(conn: &mut MysqlConnection,user_name: &str, email: &str)  {

    let new_user = NewUser {
        user_name,
        email         
    };
    diesel::insert_into(users::table).values(&new_user).execute(conn).unwrap_or_else(|x| {
        panic!("{x}");
        
    });
}
// #[memoize::memoize]
pub fn populate_database(conn: &mut MysqlConnection) {
    let users_list = HashMap::from([
        ("Eugene","eugene@gmail.com"),
        ("Sam","sam@gmail.com"),
        ("Mike","mike@gmail.com")
    ]);
    for(user_name,email) in users_list {
        diesel::insert_into(users::table).values(NewUser {
        user_name,
        email         
    }).execute(conn).unwrap_or_else(|x| {
        panic!("{x}");        
    });
    }
    println!("Database populated!");

}