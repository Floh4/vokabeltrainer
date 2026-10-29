use rusqlite::{Connection, Result};
use std::env;
const DB_NAME: &str = "vocab";

fn main() -> Result<()> {
    // check if it is already initialized
    let home_path = env::home_dir().expect("Failed to get the path to the home directory: ");
    let app_dir = home_path.clone().join(".local/share/vocab_trainer");
    if !app_dir.is_dir() {
        // app is run for the first time
        // initialize everything
    }

    // do stuff

    Ok(())
}
