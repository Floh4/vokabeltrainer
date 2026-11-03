use error::VocabError;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;
use std::env;

mod error;

const DB_NAME: &str = "vocab.db";
const DB_PATH_FROM_HOME: &str = ".local/share/vocab_trainer/";


fn initialize(dir: &str) -> Result<(), VocabError> {
    println!("No database found");
    println!("Initializing");

    let path = Path::new(dir);
    fs::create_dir_all(dir)
        .map_err(|e| VocabError::IoError(e))?;

    println!("Succesfully created app path");
    println!("Creating new Database");

    let db = Connection::open(path.join(DB_NAME))
        .map_err(|e| VocabError::DBError(e))?;

    println!("Succesfully created new database!");

    // create tables
    let size = db.execute(
        "CREATE TABLE vocab (
        id int PRIMARY KEY,
        Vocabulary VARCHAR(512),
        Translation VARCHAR(512),
        SuccesRate FLOAT(16)
        );",
        [],
    ).map_err(|e| VocabError::DBError(e))?;

    println!("Succesfully created table: {}", size);
    Ok(())
}

#[test]
fn test_initialize_fail_on_empty_db_name() {

    let result = initialize("///");
    match result {
        Err(e) => match e
        {
            VocabError::DBError(_) => assert!(true),
            _ => assert!(false, "Result should be an DBError"),
        },
        _ => assert!(false, "Result should be an error!"),
    }
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
        match initialize(app_dir.display().to_string().as_str())
        {
            Ok(_) => { println!("DB was initialized") }
            Err(e) => match e
            {
                // At this point it's not that useful, but later
                // our own Error type enables better error handling
                VocabError::DBError(_) => { eprintln!("Failed to initialize DB") }
                VocabError::IoError(_) => { eprintln!("IO error while initializing DB") }
            }
        }
    }

    // do stuff

    Ok(())
}
