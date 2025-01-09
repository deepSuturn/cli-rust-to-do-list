use clap::Parser;
use to_do::{delete_all, models::NewTodo, print_tasks};
mod env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "create")]
    command: String,

    #[arg(short, long)]
    task: Option<String>,    
    
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
            Some(task) => to_do::create_task(NewTodo { title: task.as_str(), done: args.complete }),
            None => panic!("Error: --task is required for the 'create' command."),
        },
        "mark" => to_do::update_status(args.position),
        "delete" => to_do::delete_task(args.position),
        "wipe_all" => delete_all(),
        _ => panic!("unknown command."),
    }
}
