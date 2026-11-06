use error::VocabError;
use rusqlite::{Connection, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

mod error;

const DB_NAME: &str = "vocab.db";
const DB_PATH_FROM_HOME: &str = ".local/share/vocab_trainer/";

fn create_db(path: PathBuf) -> Result<Connection, VocabError> {
    let db = Connection::open(path).map_err(|e| VocabError::DBError(e))?;

    println!("Succesfully created new database!");

    // create tables
    println!("Creating tables");

    let size = db
        .execute(
            "CREATE TABLE vocab (
        ID INTEGER PRIMARY KEY,
        Vocabulary TEXT,
        Translation TEXT,
        SuccesRate FLOAT(16),
        VocabCheckCount INTEGER,
        Corect INTEGER
        );",
            [],
        )
        .map_err(|e| VocabError::DBError(e))?;

    println!("Succesfully created table: {}", size);
    Ok(db)
}

fn initialize() -> Result<Connection, VocabError> {
    // check if it is already initialized
    let app_dir = env::home_dir()
        .expect("failed to to get the path to the home directory")
        .join(DB_PATH_FROM_HOME);
    let db_path = app_dir.join(DB_NAME);

    if !app_dir.is_dir() {
        // app is run for the first time
        // initialize everything

        println!("No database found");
        println!("Initializing");

        fs::create_dir_all(app_dir).map_err(|e| VocabError::IoError(e))?;

        println!("Succesfully created app path");
        println!("Creating new Database");

        create_db(db_path)
    } else if !db_path.exists() {
        // path exists but, not the database
        create_db(db_path)
    } else {
        let db = Connection::open(db_path).map_err(|e| VocabError::DBError(e))?;
        Ok(db)
    }
}

fn add_vocab(conn: Connection, vocab: &str, translation: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO vocab (Vocabulary, Translation, SuccesRate, VocabCheckCount, Corect)
         VALUES (?1, ?2, 0.0, 0, 0);",
        [vocab, translation],
    )?;
    Ok(())
}

fn main() -> Result<()> {
    /*
    let conn = match initialize() {
        Ok(db) => {
            println!("DB was initialized");
            db
        }
        Err(e) => match e {
            // At this point it's not that useful, but later
            // our own Error type enables better error handling.
            // Currently it wont work because it expects
            // a Connection.
            VocabError::DBError(_) => {
                eprintln!("Failed to initialize DB")
            }
            VocabError::IoError(_) => {
                eprintln!("IO error while initializing DB")
            }
        },
    };
    */

    let conn = initialize().unwrap(); // currently no better way to handle errors

    // do stuff

    Ok(())
}

// Tests
/* Wont work because the intitialize function changed
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
*/

#[test]
// run: cargo test print_db -- --nocapture
fn print_db() {
    let path = env::home_dir()
        .unwrap()
        .join(DB_PATH_FROM_HOME)
        .join(DB_NAME);
    let conn = Connection::open(path).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM vocab").unwrap();
    let mut rows = stmt.query([]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        println!("{:?}", row);
    }
}

#[test]
fn reset() {
    let path = env::home_dir().unwrap().join(DB_PATH_FROM_HOME);
    fs::remove_dir_all(path).unwrap();
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
