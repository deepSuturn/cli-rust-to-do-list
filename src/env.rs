use dotenvy::dotenv;
use std::{env, fs};

pub fn load_variables() {
    dotenv().ok();

    let migrations_dir = env::var("MIGRATIONS_DIR").expect("MIGRATIONS_DIR must be set");

    let toml_content = fs::read_to_string("diesel.toml").expect("Failed to read config.toml");

    let _ = toml_content.replace("$MIGRATIONS_DIR", &migrations_dir);
}