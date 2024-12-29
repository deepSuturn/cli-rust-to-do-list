use to_do_list::establish_connection;
use std::io::stdin;
use to_do_list::create_to_do;

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut marked = String::new();
    
    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!("\nOk! Is {title} marked!? [Y/N]\n",);
    stdin().read_line(&mut marked).unwrap();
    
    let mut done = false;

    if &marked == "Y" {
        done = true;
    }
    

    let post = create_to_do(connection, title, done);
    println!("\nSaved draft {title} with id {}", post.id);
}



