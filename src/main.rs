use error::VocabError;
use rusqlite::{Connection, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

mod error;

const DB_NAME: &str = "vocab.db";
const DB_PATH_FROM_HOME: &str = ".local/share/vocab_trainer/";

fn initialize(dir: PathBuf) -> Result<(), VocabError> {
    println!("No database found");
    println!("Initializing");

    fs::create_dir_all(dir.clone()).map_err(|e| VocabError::IoError(e))?;

    println!("Succesfully created app path");
    println!("Creating new Database");

    let db = Connection::open(dir.join(DB_NAME)).map_err(|e| VocabError::DBError(e))?;

    println!("Succesfully created new database!");

    // create tables
    let size = db
        .execute(
            "CREATE TABLE vocab (
        id int PRIMARY KEY,
        Vocabulary VARCHAR(512),
        Translation VARCHAR(512),
        SuccesRate FLOAT(16),
        VocabCheckCount int,
        Corect int
        );",
            [],
        )
        .map_err(|e| VocabError::DBError(e))?;

    println!("Succesfully created table: {}", size);
    Ok(())
}

#[test]
fn test_initialize_fail_on_empty_db_name() {
    let result = initialize("///".into());
    match result {
        Err(e) => match e {
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
    assert!(true, "{:?}", entrys);
}

#[test]
fn reset() {
    let path = env::home_dir().unwrap().join(DB_PATH_FROM_HOME);
    fs::remove_dir_all(path).unwrap();
}

fn add_vocab(conn: Connection, vocab: &str, translation: &str) -> Result<()> {
    conn.execute("INSERT INTO vocab (Vocabulary, Translation, SuccesRate, VocabCheckCount, Corect)
                  VALUES (?1, ?2, 0.0, 0, 0);", [vocab, translation])?;
    Ok(())
}

#[test]
fn test_add_vocab() {
    let path = env::home_dir()
        .unwrap()
        .join(DB_PATH_FROM_HOME)
        .join(DB_NAME);
    let conn = Connection::open(path).unwrap();
    let result = add_vocab(conn, "Dog", "Hund");
    match result {
        Ok(()) => assert!(true),
        Err(e) => assert!(false, "couldnt execute sql statement: {}", e),
    }
}

fn main() -> Result<()> {
    // check if it is already initialized
    let app_dir = env::home_dir()
        .expect("failed to to get the path to the home directory")
        .join(DB_PATH_FROM_HOME);

    if !app_dir.is_dir() {
        // app is run for the first time
        // initialize everything
        match initialize(app_dir) {
            Ok(_) => {
                println!("DB was initialized")
            }
            Err(e) => match e {
                // At this point it's not that useful, but later
                // our own Error type enables better error handling
                VocabError::DBError(_) => {
                    eprintln!("Failed to initialize DB")
                }
                VocabError::IoError(_) => {
                    eprintln!("IO error while initializing DB")
                }
            },
        }
    }

    // do stuff

    Ok(())
}
