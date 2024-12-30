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
            format_task();
            answer
        }
        2 => {
            print!("Task Id: ");
            let mut id = String::new();
            stdin().read_line(&mut id).expect("Failed to read answer.");
            update_status(id.trim().parse().expect("Expected a valid id number."), read_tasks());
            answer
        },
        3 => {
            print!("Task Id: ");
            let mut id = String::new();
            stdin().read_line(&mut id).expect("Failed to read answer.");
            delete_task(id.trim().parse().expect("Expected a valid id."), read_tasks());
            answer
        }
        _ => {
            println!("No answer available");
            return -1
        }
    }
}

pub fn format_task() {
    let connection = &mut establish_connection();
    let mut title = String::new();
    let mut done = String::new();
    
    println!("What's the name of your task?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("\nOk! Is {title} done? [Y/N]",);
    stdin().read_line(&mut done).unwrap();
    let done = match done.as_str() {"Y" => true, _ => false,};

    create_task(connection, title, done);
    println!("Task saved!");
}

pub fn create_task(conn: &mut PgConnection, title: &str, done: bool){
    use crate::schema::to_do;

    let task = NewTodo {title, done};
    
    diesel::insert_into(to_do::table)
        .values(&task)
        .returning(ToDo::as_returning())
        .get_result(conn)
        .expect("Error saving new task");
}

pub fn read_tasks() -> Vec<ToDo> {
    use crate::schema::to_do::dsl::*;
    let connection= &mut establish_connection();
    let tasks = to_do
                    .select(ToDo::as_select())
                    .load(connection)
                    .expect("Error while loading tasks");
    tasks
}

pub fn update_status(position : usize, tasks : Vec<ToDo>) {
    use crate::schema::to_do::dsl::{to_do, done};
    let connection = &mut establish_connection();
    let task = match tasks[position].done {
        false => diesel::update(to_do.find(tasks[position].id)).set(done.eq(true)).returning(ToDo::as_returning()).get_result(connection).unwrap(),
        true => diesel::update(to_do.find(tasks[position].id)).set(done.eq(false)).returning(ToDo::as_returning()).get_result(connection).unwrap(),
    };
    
    println!("Completed Task: {}", task.title);
}

pub fn delete_task(position : usize, tasks : Vec<ToDo>) {
    use crate::schema::to_do::dsl::*;

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(to_do.filter(id.eq(tasks[position].id)))
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
