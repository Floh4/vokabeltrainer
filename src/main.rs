use error::VocabError;
use rusqlite::{Connection, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

mod error;

trait Datastore {
    fn add_vocab(&mut self, vocab: &str, translation: &str) -> Result<(), VocabError>;
    fn update(&mut self, id: u32) -> Result<(), VocabError>;
    fn get_solution(&mut self, id: u32) -> Result<String, VocabError>;
}

struct SqlLiteDatastore {
    conn: Connection,
}

impl SqlLiteDatastore {
    const DB_NAME: &str = "vocab.db";
    const DB_PATH_FROM_HOME: &str = ".local/share/vocab_trainer/";

    fn create() -> Result<Self, VocabError> {
        Self::create_from_path(None, None)
    }
    fn create_from_path(path: Option<PathBuf>, db_name: Option<&str>) -> Result<Self, VocabError> {
        let app_path = match path {
            Some(p) => p,
            None => env::home_dir()
                .ok_or(VocabError::General(
                    "failed to to get the path to the home directory".to_string(),
                ))?
                .join(SqlLiteDatastore::DB_PATH_FROM_HOME),
        };
        let db_name = db_name.unwrap_or(SqlLiteDatastore::DB_NAME);
        let conn = SqlLiteDatastore::initialize(app_path, db_name)?;
        Ok(SqlLiteDatastore { conn })
    }

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

    fn initialize(app_dir: PathBuf, db_name: &str) -> Result<Connection, VocabError> {
        let db_path = app_dir.join(db_name);
        if !app_dir.is_dir() {
            // app is run for the first time
            // initialize everything

            println!("No database found");
            println!("Initializing");

            fs::create_dir_all(app_dir).map_err(|e| VocabError::IoError(e))?;

            println!("Succesfully created app path");
            println!("Creating new Database");

            SqlLiteDatastore::create_db(db_path)
        } else if !db_path.exists() {
            // path exists but, not the database
            SqlLiteDatastore::create_db(db_path)
        } else {
            let db = Connection::open(db_path).map_err(|e| VocabError::DBError(e))?;
            Ok(db)
        }
    }
}

impl Datastore for SqlLiteDatastore {
    fn add_vocab(&mut self, vocab: &str, translation: &str) -> Result<(), VocabError> {
        match self.conn.query_row(
            "SELECT Vocabulary
             FROM vocab
             WHERE Vocabulary = ?1",
            [vocab],
            |row| row.get::<usize, String>(0),
        ) {
            Ok(_) =>
            // vocab exists
            {
                Err(VocabError::UniqueVocab(
                    "This vocab already exists".to_string(),
                ))
            }
            Err(err) => match err {
                rusqlite::Error::QueryReturnedNoRows =>
                // vocab doesnt exist
                {
                    self.conn
                    .execute(
                        "INSERT INTO vocab (Vocabulary, Translation, SuccesRate, VocabCheckCount, Corect)
                         VALUES (?1, ?2, 0.0, 0, 0);",
                        [vocab, translation],
                    )
                    .map_err(|e| VocabError::DBError(e))?;
                    Ok(())
                }

                e =>
                // sql error
                {
                    Err(VocabError::DBError(e))
                }
            },
        }
    }

    fn get_solution(&mut self, id: u32) -> Result<String, VocabError> {
        match self.conn.query_row(
            "SELECT Translation
             FROM vocab
             WHERE ID = ?1",
            [id],
            |row| row.get::<usize, String>(0),
        ) {
            Ok(v) => {
                self.update(id)?;
                Ok(v)
            }

            Err(err) => match err {
                rusqlite::Error::QueryReturnedNoRows => Err(VocabError::General(
                    "Found no Record with that id".to_string(),
                )), // This wont need its own error type, because this function wont be using User input
                e => Err(VocabError::DBError(e)),
            },
        }
    }

    fn update(&mut self, id: u32) -> Result<(), VocabError> {
        // add 1 to VocabCheckCount
        self.conn
            .execute(
                "UPDATE vocab
                 SET VocabCheckCount = VocabCheckCount + 1
                 WHERE ID = ?1",
                [id],
            )
            .map_err(|e| VocabError::DBError(e))?;
        let count: u32 = self
            .conn
            .query_row(
                "SELECT VocabCheckCount
                 FROM vocab
                 WHERE ID = ?1",
                [id],
                |row| row.get(0),
            )
            .map_err(|e| VocabError::DBError(e))?;
        let corect: u32 = self
            .conn
            .query_row(
                "SELECT Corect
                 FROM vocab
                 WHERE ID = ?1",
                [id],
                |row| row.get(0),
            )
            .map_err(|e| VocabError::DBError(e))?;
        // calculate succes rate
        let succes_rate: f32 = corect as f32 / count as f32;
        // update succes rate
        self.conn
            .execute(
                "UPDATE vocab
                 SET SuccesRate = ?1
                 WHERE ID = ?2",
                rusqlite::params![succes_rate, id],
            )
            .map_err(|e| VocabError::DBError(e))?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let datastore = SqlLiteDatastore::create();
    // do stuff

    Ok(())
}

// Tests
#[test]
fn test_create_db() {
    assert!(SqlLiteDatastore::create().is_ok());
}

#[test]
// run: cargo test print_db -- --nocapture
fn print_db() {
    let datastore = SqlLiteDatastore::create_from_path(None, None).unwrap();
    let mut stmt = datastore.conn.prepare("SELECT * FROM vocab").unwrap();
    let mut rows = stmt.query([]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        println!("{:?}", row);
    }
}

#[test]
fn reset() {
    let path = env::home_dir()
        .unwrap()
        .join(SqlLiteDatastore::DB_PATH_FROM_HOME);
    fs::remove_dir_all(path).unwrap();
}

#[test]
fn test_add_vocab() {
    let mut datastore = SqlLiteDatastore::create().unwrap();
    let result = datastore.add_vocab("Dog", "Hund");
    match result {
        Ok(()) => assert!(true),
        Err(e) => match e {
            VocabError::UniqueVocab(e) => assert!(false, "Error: {}", e),
            VocabError::DBError(e) => assert!(false, "Database Error: {}", e),
            e => assert!(false, "Different Error occurred: {}", e),
        },
    }
}

#[test]
fn test_get_solution() {
    let mut datastore = SqlLiteDatastore::create().unwrap();
    datastore.add_vocab("Door", "Tür").unwrap();
    let _ = datastore.get_solution(0).is_ok();
}

#[test]
fn test_update() {
    let mut datastore = SqlLiteDatastore::create().unwrap();
    datastore.add_vocab("Dog", "Hund").unwrap();
    let id = datastore
        .conn
        .query_row(
            "SELECT ID
             FROM vocab
             WHERE Vocabulary = ?1",
            ["Dog"],
            |row| row.get(0),
        )
        .unwrap();
    datastore.update(id).unwrap();
}

#[test]
// run: cargo test list -- --nocapture
fn list() {
    // lists all available tests
    println!("list                              list all tests");
    println!("  print_db                        ! run: cargo test -- --nocapture");
    println!("  reset                           ! use carefully: deletes the database");
    println!("  test_add_vocab                  adds new vocabulary");
    println!("  test_create_db                  creates new db");
    println!("  test_get_solution               get the translation of the word");
    println!("  test_update                     update the count and succes rate");
    assert!(true);
}
