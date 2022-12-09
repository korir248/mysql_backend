use self::models::*;
use diesel::prelude::*;
use mysql_backend::*;

fn main() {
    use self::schema::users::dsl::*;

    let conn = &mut establish_connection();
    
    // add_user(conn, "Hariette Maina", "hariette@gmail.com");
    let results = users
    .filter(admin.eq(false))
    .limit(5)
    .load::<User>(conn)
    .expect("Error loading users");
    
    if results.len() > 0 {
        if results.len() == 1 {
            println!("Displaying {} user", results.len());
        }else {
            println!("Displaying {} users", results.len());
        }
        for user in results {
            println!("{:#?}", user);
        }
    }else{
        populate_database(conn);
    }

}