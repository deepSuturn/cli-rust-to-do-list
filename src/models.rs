use diesel::prelude::*;
use crate::schema::to_do;

#[derive(Clone)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = to_do)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ToDo {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(Insertable)]
#[diesel(table_name = to_do)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub done: bool,
}
