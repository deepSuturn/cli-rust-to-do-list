use to_do_list;

fn main() {
    let mut sentinel = 0;
    while sentinel != -1 {
        to_do_list::print_tasks(to_do_list::read_todos());
        sentinel = to_do_list::load_menu();
    }
}
