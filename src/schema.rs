// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        user_name -> Varchar,
        email -> Nullable<Varchar>,
    }
}
