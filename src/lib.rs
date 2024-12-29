use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewTodo, ToDo};
use std::env::{self, args};
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_to_do(conn: &mut PgConnection, title: &str, done: bool) -> ToDo {
    use crate::schema::to_do;

    let new_todo = NewTodo {title, done};
    
    return diesel::insert_into(to_do::table)
        .values(&new_todo)
        .returning(ToDo::as_returning())
        .get_result(conn)
        .expect("Error saving new todo")
}

pub fn read_todos() {
    use crate::schema::to_do::dsl::*;


    let connection= &mut establish_connection();
    let results = to_do
                    .filter(done.eq(true))
                    .limit(5)
                    .select(ToDo::as_select())
                    .load(connection)
                    .expect("Error while loading to_dos");


    println!("Displaying {} to_dos", results.len());
    for to_do_item in results {
        println!("{}", to_do_item.title);
        println!("{}", to_do_item.done);
    }
}

pub fn update_mark_todo() {
    use crate::schema::to_do::dsl::{to_do, done};

    let id = args().nth(1).expect("to mark a todo it requires a to_do id").parse::<i32>().expect("invalid ID");

    let connection = &mut establish_connection();

    let to_do_item = diesel::update(to_do.find(id)).set(done.eq(true)).returning(ToDo::as_returning()).get_result(connection).unwrap();
    print!("marked to_do {}", to_do_item.title);
}

pub fn delete_todo() {
    use crate::schema::to_do::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(to_do.filter(title.like(pattern))).execute(connection).expect("Expect deleting posts");

    println!("Deleted {} todos", num_deleted);
}
 
