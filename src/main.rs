use clap::Parser;
use to_do_list::{self, delete_all, models::NewTodo, print_tasks};
use std::fmt::Debug;
mod env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    task: Option<String>,
    
    #[arg(short, long, default_value = "create")]
    command: String,
    
    #[arg(short, long, default_value_t = 0)]
    position: usize,

    #[arg(long)]
    complete: bool,
}

fn main() {
    env::load_variables();
    let args = Args::parse();

    match args.command.as_str() {
        "show" => print_tasks(),
        "create" => match args.task {
            Some(task) => to_do_list::create_task(NewTodo { title: task.as_str(), done: args.complete }),
            None => println!("Error: --task is required for the 'create' command."),
        },
        "mark" => to_do_list::update_status(args.position),
        "delete" => to_do_list::update_status(args.position),
        "wipe_all" => delete_all(),
        _ => println!("unknown command."),
    }
}
