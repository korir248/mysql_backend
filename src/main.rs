use self::models::*;
use diesel::prelude::*;
use mysql_backend::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(admin.eq(false))
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:#?}", user);
    }
}