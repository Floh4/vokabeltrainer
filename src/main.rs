use rusqlite::{Connection, Result};
use std::env;
use std::fs;
use std::path::Path;

const DB_NAME: &str = "vocab.db";

fn initialize(dir: &str) -> Result<()> {
    println!("No database found");
    println!("Initializing");
    let path = Path::new(dir);
    fs::create_dir_all(dir).expect("Failed to create the app directory");
    println!("Creating new Database");
    let db = Connection::open(path.join(DB_NAME))?;

    Ok(())
}

fn main() -> Result<()> {
    // check if it is already initialized
    let app_dir = env::home_dir()
        .expect("failed to to get the path to the home directory")
        .join(".local/share/vocab_trainer/");
    println!("{}", app_dir.display());

    if !app_dir.is_dir() {
        // app is run for the first time
        // initialize everything
        initialize(app_dir.display().to_string().as_str());
    }

    // do stuff

    Ok(())
}
