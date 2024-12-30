use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewTodo, ToDo};
use core::panic;
use std::{env::{self}, io::stdin};
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn load_menu() -> i32 {
    println!("\n[1] - Create Task\n[2] - Mark Task\n[3] - Delete Task");
    let mut answer = String::new();

    stdin().read_line(&mut answer).expect("Failed to read answer.");

    let answer: i32 = answer.trim().parse().expect("Expected a number answer");

    match answer {
        1 => {
            request_to_do();
            answer
        }
        2 => {
            print!("Task Id: ");
            let mut id = String::new();
            stdin().read_line(&mut id).expect("Failed to read answer.");
            update_mark_todo(id.trim().parse().expect("Expected a valid id."), read_todos());
            answer
        },
        3 => {
            print!("Task Id: ");
            let mut id = String::new();
            stdin().read_line(&mut id).expect("Failed to read answer.");
            delete_todo(id.trim().parse().expect("Expected a valid id."), read_todos());
            answer
        }
        _ => {
            println!("No answer available");
            return -1
        }
    }
}

pub fn request_to_do() {
    let connection = &mut establish_connection();
    
    let mut title = String::new();
    let mut marked = String::new();
    
    println!("What's the name of your task?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!("\nOk! Is {title} done? [Y/N]",);
    stdin().read_line(&mut marked).unwrap();
    
    let mut done = false;

    if &marked == "Y" {
        done = true;
    }
    

    create_to_do(connection, title, done);
    println!("Task saved!");
}

pub fn create_to_do(conn: &mut PgConnection, title: &str, done: bool){
    use crate::schema::to_do;

    let new_todo = NewTodo {title, done};
    
    diesel::insert_into(to_do::table)
        .values(&new_todo)
        .returning(ToDo::as_returning())
        .get_result(conn)
        .expect("Error saving new todo");
}

pub fn read_todos() -> Vec<ToDo> {
    use crate::schema::to_do::dsl::*;
    let connection= &mut establish_connection();
    let results = to_do
                    .select(ToDo::as_select())
                    .load(connection)
                    .expect("Error while loading to_dos");
    results
}

pub fn update_mark_todo(id_task : usize, results : Vec<ToDo>) {
    use crate::schema::to_do::dsl::{to_do, done};
    let connection = &mut establish_connection();
    let todo_item = match results[id_task].done {
        false => diesel::update(to_do.find(results[id_task].id)).set(done.eq(true)).returning(ToDo::as_returning()).get_result(connection).unwrap(),
        true => diesel::update(to_do.find(results[id_task].id)).set(done.eq(false)).returning(ToDo::as_returning()).get_result(connection).unwrap(),
    };
    
    println!("Completed Task: {}", todo_item.title);
}

pub fn delete_todo(id_task : usize, results : Vec<ToDo>) {
    use crate::schema::to_do::dsl::*;

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(to_do.filter(id.eq(results[id_task].id)))
        .execute(connection)
        .expect("Error deleting the task");

    println!("Deleted Tasks: {}", num_deleted);
}

pub fn print_tasks(results : Vec<ToDo>) {
    if results.len() == 0 {
        println!("No tasks available.")
    } else {
        println!("You have {} tasks!", results.len());

        

        for i in 0..results.len() {
            match results[i].done {
                true => println!("({}){} - Done", i, results[i].title),
                false => println!("({}){} - []", i, results[i].title),
            }
        }
    }
}
