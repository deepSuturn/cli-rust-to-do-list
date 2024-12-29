// @generated automatically by Diesel CLI.

diesel::table! {
    to_do (id) {
        id -> Int4,
        title -> Varchar,
        done -> Bool,
    }
}
