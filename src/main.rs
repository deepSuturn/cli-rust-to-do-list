use to_do_list;
mod env;

fn main() {
    env::load_variables();

    let mut sentinel = 0;
    while sentinel != -1 {
        to_do_list::print_tasks();
        sentinel = to_do_list::load_menu();
    }
}
