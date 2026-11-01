use rusqlite::{Connection, Result};
use std::env;
use std::fs;
use std::path::Path;

const DB_NAME: &str = "vocab.db";
const DB_PATH_FROM_HOME: &str = ".local/share/vocab_trainer/";

fn initialize(dir: &str) -> Result<()> {
    println!("No database found");
    println!("Initializing");

    let path = Path::new(dir);
    fs::create_dir_all(dir).expect("Failed to create the app directory");

    println!("Succesfully created app path");
    println!("Creating new Database");

    let db = Connection::open(path.join(DB_NAME))?;

    println!("Succesfully created new database!");

    // create tables
    match db.execute(
        "CREATE TABLE vocab (
        id int PRIMARY KEY,
        Vocabulary VARCHAR(512),
        Translation VARCHAR(512),
        SuccesRate FLOAT(16)
        );",
        [],
    ) {
        Ok(v) => println!("Succesfully created table: {}", v),
        Err(e) => println!("Failed to create table: {}", e),
    }

    Ok(())
}

#[test]
fn print_db() {
    let path = env::home_dir()
        .unwrap()
        .join(DB_PATH_FROM_HOME)
        .join(DB_NAME);
    let conn = Connection::open(path).unwrap();

    let mut stmt = conn.prepare("SELECT * FROM vocab").unwrap();
    let mut rows = stmt.query([]).unwrap();

    let mut entrys: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        entrys.push(row.get(0).unwrap());
    }
    println!("{:?}", entrys);
}

#[test]
fn reset() {
    let path = env::home_dir().unwrap().join(DB_PATH_FROM_HOME);
    fs::remove_dir_all(path).unwrap();
}

fn main() -> Result<()> {
    // check if it is already initialized
    let app_dir = env::home_dir()
        .expect("failed to to get the path to the home directory")
        .join(DB_PATH_FROM_HOME);

    if !app_dir.is_dir() {
        // app is run for the first time
        // initialize everything
        initialize(app_dir.display().to_string().as_str()).expect("Failed to initialize");
    }

    // do stuff

    Ok(())
}
