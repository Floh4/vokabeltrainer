use crate::vocabulary::Vocabulary;
use datastore::{Datastore, SqlLiteDatastore};
use rusqlite::Result;
use std::io;

mod datastore;
mod error;
mod vocabulary;

fn main() -> Result<()> {
    let mut datastore = SqlLiteDatastore::create().unwrap();
    // do stuff
    let vocab = Vocabulary {
        vocabulary: String::from("Street"),
        translation: String::from("Strasse"),
        ..Default::default()
    };
    datastore.add_vocab(vocab).unwrap();
    let mut vocabulary = datastore
        .id_to_vocabulary(datastore.random_id().unwrap())
        .unwrap();
    println!("Enter translation for {}", vocabulary.vocabulary);
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    println!("Correctness: {}", vocabulary.check(buffer.trim()).unwrap());
    println!("Overall correctness: {}", vocabulary.correct);
    datastore.update_vocab(vocabulary).unwrap();
    Ok(())
}
