use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewTodo, ToDo};
use std::{env::{self}, io::stdin};
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn load_menu() -> isize {
    println!("\n[1] - Create Task\n[2] - Mark Task\n[3] - Delete Task");
    let menu_response = get_number_input();
    match menu_response {
        1 => {
            create_task(NewTodo { title : create_title().as_str(), done : ask_done(), });
        },
        2 => {
            println!("Task Id: ");
            update_status(get_number_input(), read_tasks());
        },
        3 => {
            println!("Task Id: ");
            delete_task(get_number_input(), read_tasks());
        }
        _ => {
            println!("No answer available");
            return -1
        }
    }
    0
}

pub fn create_task(task : NewTodo){
    use crate::schema::to_do;
    let connection= &mut establish_connection();
    diesel::insert_into(to_do::table)
        .values(&task)
        .returning(ToDo::as_returning())
        .get_result(connection)
        .expect("Error saving new task");
}

pub fn read_tasks() -> Vec<ToDo> {
    use crate::schema::to_do::dsl::*;
    let connection= &mut establish_connection();
    let tasks = to_do.select(ToDo::as_select()).load(connection).expect("Error while loading tasks");
    tasks
}

pub fn update_status(position_list : usize, tasks : Vec<ToDo>) {
    use crate::schema::to_do::dsl::{to_do, done};
    let connection = &mut establish_connection();
    let task = match tasks[position_list].done {
        false => diesel::update(to_do.find(tasks[position_list].id)).set(done.eq(true)).returning(ToDo::as_returning()).get_result(connection).unwrap(),
        true => diesel::update(to_do.find(tasks[position_list].id)).set(done.eq(false)).returning(ToDo::as_returning()).get_result(connection).unwrap(),
    };
    println!("Completed Task: {}", task.title);
}

pub fn delete_task(position_list : usize, tasks : Vec<ToDo>) {
    use crate::schema::to_do::dsl::*;
    let connection = &mut establish_connection();

    let num_deleted = diesel::delete(to_do.filter(id.eq(tasks[position_list].id)))
        .execute(connection)
        .expect("Error deleting the task");

    println!("Deleted Tasks: {}", num_deleted);
}

pub fn print_tasks(tasks : Vec<ToDo>) {
    match tasks.len() {
        0 => println!("No tasks available!"),
        _ =>{  println!("You have {} tasks:", tasks.len());
                for i in 0..tasks.len() {
                    if tasks[i].done { println!("({}){} - Done", i, tasks[i].title) } else { println!("({}){} - []", i, tasks[i].title) };
                }
        }
    }
}

pub fn get_number_input() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Expected a number") 
}

pub fn create_title() -> String {
    let mut title = String::new();
    println!("What's the name of your task?");
    stdin().read_line(&mut title).unwrap();
    title.trim().to_string()
}

pub fn ask_done() -> bool {
    println!("\nOk! Is the task done? [Y/N]",);
    let mut done = String::new();
    stdin().read_line(&mut done).unwrap();
    if done.trim().to_uppercase().as_str() == "Y" {true} else {false}
}
