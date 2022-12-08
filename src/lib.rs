pub mod models;
pub mod schema;
// use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::{mysql::MysqlConnection, Connection};


pub fn establish_connection() -> () {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        ()
}
