use crate::error::VocabError;
use crate::vocabulary::Vocabulary;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use rusqlite::{Connection, Row, params};
use std::path::PathBuf;
use std::{env, fs};

pub trait Datastore {
    fn add_vocab(&mut self, vocab: Vocabulary) -> rusqlite::Result<(), VocabError>;
    //fn add_vocab(&mut self, vocab: &str, translation: &str, language: &str, vocab_info: Option<&str>, translation_info: Option<&str>, info: Option<&str>) -> rusqlite::Result<(), VocabError>;
    fn get_solution(&mut self, word: String) -> rusqlite::Result<String, VocabError>;
    fn id_to_vocabulary(&mut self, id: u32) -> Result<Vocabulary, VocabError>;
    fn random_id(&self) -> Result<u32, VocabError>;
    fn update_vocab(&mut self, vocab: Vocabulary) -> rusqlite::Result<(), VocabError>;
}

impl From<&Row<'_>> for Vocabulary {
    fn from(row: &Row) -> Self {
        Vocabulary {
            id: row.get(0).unwrap(),
            vocabulary: row.get(1).unwrap(),
            translation: row.get(2).unwrap(),
            vocab_check_count: row.get(3).unwrap(),
            correct: row.get(4).unwrap(),
            language: row.get(5).unwrap(),
            group: row.get(6).unwrap(),
            vocab_info: row.get(7).unwrap(),
            translation_info: row.get(8).unwrap(),
            info: row.get(9).unwrap(),
        }
    }
}

pub struct SqlLiteDatastore {
    conn: Connection,
}

impl SqlLiteDatastore {
    const DB_NAME: &str = "vocab.db";
    const DB_PATH_FROM_HOME: &str = ".local/share/vocab_trainer/";

    pub(crate) fn create() -> rusqlite::Result<Self, VocabError> {
        Self::create_from_path(None, None)
    }

    fn create_from_path(
        path: Option<PathBuf>,
        db_name: Option<&str>,
    ) -> rusqlite::Result<Self, VocabError> {
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

    fn create_db(path: PathBuf) -> rusqlite::Result<Connection, VocabError> {
        let db = Connection::open(path).map_err(|e| VocabError::DBError(e))?;
        println!("Succesfully created new database!");

        // create tables
        println!("Creating tables");
        let size = db
            .execute(
                r#"CREATE TABLE vocab (
                 ID INTEGER PRIMARY KEY,
                 Vocabulary TEXT,
                 Translation TEXT,
                 VocabCheckCount INTEGER,
                 Corect FLOAT(16),
                 Language TEXT NOT NULL,
                 "Group" TEXT,
                 VocabInfo TEXT,
                 TranslationInfo TEXT,
                 Info TEXT
                 );"#,
                [],
            )
            .map_err(|e| VocabError::DBError(e))?;
        println!("Succesfully created table: {}", size);
        Ok(db)
    }

    fn initialize(app_dir: PathBuf, db_name: &str) -> rusqlite::Result<Connection, VocabError> {
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
    fn add_vocab(&mut self, vocab: Vocabulary) -> rusqlite::Result<(), VocabError> {
        //fn add_vocab(&mut self, vocab: &str, translation: &str, language: &str, vocab_info: Option<&str>, translation_info: Option<&str>, info: Option<&str>) -> rusqlite::Result<(), VocabError> {
        match self.conn.query_row(
            "SELECT Vocabulary
             FROM vocab
             WHERE Vocabulary = ?1",
            [&vocab.vocabulary],
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
                            r#"INSERT INTO vocab (Vocabulary, Translation, VocabCheckCount, Corect, Language, "Group",  VocabInfo, TranslationInfo, Info)
                             VALUES (?1, ?2, 0, 0.0, ?3, ?4, ?5, ?6, ?7);"#,
                            params![vocab.vocabulary, vocab.translation, vocab.language, vocab.group, vocab.vocab_info, vocab.translation_info, vocab.info],
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

    fn get_solution(&mut self, word: String) -> rusqlite::Result<String, VocabError> {
        match self.conn.query_row(
            "SELECT Translation
             FROM vocab
             WHERE Vocabulary = ?1",
            [word],
            |row| row.get::<usize, String>(0),
        ) {
            Ok(v) => {
                //self.update(id)?;
                Ok(v)
            }

            Err(err) => match err {
                rusqlite::Error::QueryReturnedNoRows => {
                    Err(VocabError::General("Couldnt find the word".to_string()))
                } // This wont need its own error type, because this function wont be using User input
                e => Err(VocabError::DBError(e)),
            },
        }
    }

    fn id_to_vocabulary(&mut self, id: u32) -> Result<Vocabulary, VocabError> {
        let mut vocabulary: Vocabulary = self
            .conn
            .query_row(
                "SELECT ID, Vocabulary, Translation, VocabCheckCount, Corect
                 FROM vocab
                 WHERE ID = ?1",
                [id],
                |row| Ok(row.into()),
            )
            .map_err(|e| VocabError::DBError(e))?;

        Ok(vocabulary)
    }

    fn random_id(&self) -> Result<u32, VocabError> {
        let entrys_amount: u32 = self
            .conn
            .query_row(
                "SELECT ID
                 FROM vocab
                 ORDER BY ID DESC",
                [],
                |row| row.get(0),
            )
            .map_err(|e| VocabError::DBError(e))?;
        let mut get_corect = self
            .conn
            .prepare(
                "SELECT Corect
                 FROM vocab
                 WHERE ID = ?1",
            )
            .map_err(|e| VocabError::DBError(e))?;
        let mut get_count = self
            .conn
            .prepare(
                "SELECT VocabCheckCount
                 FROM vocab
                 WHERE ID = ?1",
            )
            .map_err(|e| VocabError::DBError(e))?;
        loop {
            // select a random id
            let id = rand::random_range(1..=entrys_amount);
            let corect: f32 = get_corect
                .query_row([id], |row| row.get(0))
                .map_err(|e| VocabError::DBError(e))?;
            let count: f32 = get_count
                .query_row([id], |row| row.get(0))
                .map_err(|e| VocabError::DBError(e))?;
            let succes_rate: f32 = if count == 0.0 {
                // the vocab was never checked before, so it will get selected
                0.0
            } else {
                corect / count as f32
            };
            // use succes rate to make probability lower, when the succes rate is high
            if rand::random_bool((1.0 - succes_rate).into()) {
                return Ok(id);
            }
        }
    }

    fn update_vocab(&mut self, vocab: Vocabulary) -> rusqlite::Result<(), VocabError> {
        self.conn
            .execute(
                "UPDATE vocab
                 SET Vocabulary = ?2,
                 Translation = ?3,
                 VocabCheckCount = ?4,
                 Corect = ?5
                 WHERE ID = ?1",
                [
                    u32::to_string(&vocab.id),
                    vocab.vocabulary,
                    vocab.translation,
                    i64::to_string(&vocab.vocab_check_count),
                    f32::to_string(&vocab.correct),
                ],
            )
            .map_err(|e| VocabError::DBError(e))?;
        Ok(())
    }
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
    let vocab = Vocabulary {
        vocabulary: String::from("Dog"),
        translation: String::from("Hund"),
        ..Default::default()
    };
    let result = datastore.add_vocab(vocab);
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
    let vocab = Vocabulary {
        vocabulary: String::from("Dog"),
        translation: String::from("Hund"),
        ..Default::default()
    };
    datastore.add_vocab(vocab).unwrap();
    assert_eq!("Tür", datastore.get_solution("Door".to_string()).unwrap());
}

#[test]
// run: cargo test test_random_id -- --nocapture
fn test_random_id() {
    let datastore = SqlLiteDatastore::create().unwrap();
    let rid = datastore.random_id().unwrap();
    println!("{}", rid);
}

#[test]
// run: cargo test list -- --nocapture
fn list() {
    // lists all available tests
    println!("list                              list all tests");
    println!("  print_db                        ! run: cargo test print_db -- --nocapture");
    println!("  reset                           ! use carefully: deletes the database");
    println!("  test_add_vocab                  adds new vocabulary");
    println!("  test_create_db                  creates new db");
    println!("  test_get_solution               get the translation of the word");
    println!(
        "  test_random_id                  get a random id ! run: cargo test test_random_id -- --nocapture"
    );
    println!("  (test_update                     update the count)");
    assert!(true);
}
