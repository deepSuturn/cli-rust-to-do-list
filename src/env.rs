use dotenvy::dotenv;
use std::{env, fs};
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn load_variables() {
    dotenv().ok();

    let migrations_dir = env::var("MIGRATIONS_DIR").expect("MIGRATIONS_DIR must be set");

    let toml_content = fs::read_to_string("/home/suturn/Documentos/Projetos/cli-to-do-list/to-do-list/diesel.toml").expect("Failed to read config.toml");

    let _ = toml_content.replace("$MIGRATIONS_DIR", &migrations_dir);

    if database_url.starts_with("sqlite://") {
        Box::new(SqliteConnection::establish(&database_url[9..]).expect("Error connecting to SQLite"))
    } else {
        Box::new(PgConnection::establish(&database_url).expect("Error connecting to PostgreSQL"))
    }
}