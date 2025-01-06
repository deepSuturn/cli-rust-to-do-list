use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewTodo, ToDo};
use std::env::{self};
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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

pub fn update_status(position_list : usize) {
    use crate::schema::to_do::dsl::{to_do, done};
    let connection = &mut establish_connection();

    let tasks = read_tasks();
    let task = match tasks[position_list].done {
        false => diesel::update(to_do.find(tasks[position_list].id)).set(done.eq(true)).returning(ToDo::as_returning()).get_result(connection).unwrap(),
        true => diesel::update(to_do.find(tasks[position_list].id)).set(done.eq(false)).returning(ToDo::as_returning()).get_result(connection).unwrap(),
    };
    println!("Completed Task: {}", task.title);
}

pub fn delete_task(position_list : usize) {
    use crate::schema::to_do::dsl::*;
    let connection = &mut establish_connection();

    let tasks = read_tasks();

    let num_deleted = diesel::delete(to_do.filter(id.eq(tasks[position_list].id)))
        .execute(connection)
        .expect("Error deleting the task");

    println!("Deleted Tasks: {}", num_deleted);
}

pub fn delete_all() {
    use crate::schema::to_do::dsl::*;
    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(to_do)
        .execute(connection)
        .expect("Error deleting all tasks");
    println!("Deleted {} tasks", num_deleted);
}

pub fn print_tasks() {
    let tasks = read_tasks();
    match tasks.len() {
        0 => println!("No tasks available!"),
        _ =>{  println!("You have {} tasks:", tasks.len());
                for i in 0..tasks.len() {
                    if tasks[i].done { println!("({}){} - Done", i, tasks[i].title) } else { println!("({}){} - []", i, tasks[i].title) };
                }
        }
    }
}

//WARNING: These tests need to be reviewed due to caching and the random order of tests.
#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    fn test_establish_connection_success() {
        dotenvy::from_filename(".env.test").ok();
        let connection = std::panic::catch_unwind(|| {
            establish_connection()
        });

        assert!(connection.is_ok(), "Connection to the database failed");
    }

    

    #[test]
    fn test_create_task() {
        delete_all();
        let mock_test = NewTodo { title: "Test Task", done: false };
        create_task(mock_test);

        let tasks = read_tasks();
        assert_eq!(tasks[tasks.len()-1].title, "Test Task");
        delete_all();
    }

    #[test]
    fn test_read_tasks() {
        delete_all();
        create_task(NewTodo { title: "Task 1", done: false });
        create_task(NewTodo { title: "Task 2", done: true });

        let tasks = read_tasks();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].title, "Task 1");
        assert!(tasks[1].done);
        delete_all();
    }

    #[test]
    fn test_update_status() {
        delete_all();
        create_task(NewTodo { title: "Task to Update", done: false });

        update_status(0);
        let updated_tasks = read_tasks();
        assert!(updated_tasks[0].done);

        update_status(0);
        let updated_tasks = read_tasks();
        assert!(!updated_tasks[0].done);
        delete_all();
    }

    #[test]
    fn test_delete_task() {
        delete_all();
        create_task(NewTodo { title: "Task to Delete", done: false });
        delete_task(0);
        let remaining_tasks = read_tasks();
        assert!(remaining_tasks.is_empty());
        delete_all();
    }

    

}