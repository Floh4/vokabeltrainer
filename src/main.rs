use std::io;
use datastore::{Datastore, SqlLiteDatastore};
use rusqlite::Result;

mod datastore;
mod error;
mod vocabulary;

fn main() -> Result<()> {
    let mut datastore = SqlLiteDatastore::create_in_memory().unwrap();
    // do stuff
    datastore.add_vocab("Street", "Strasse").unwrap();
    datastore.add_vocab("Neighbour", "Nachbar").unwrap();
    datastore.add_vocab("Cat", "Katze").unwrap();
    loop {
        let mut vocabulary = datastore
            .id_to_vocabulary(datastore.random_id().unwrap())
            .unwrap();
        println!("Enter translation for {}", vocabulary.vocabulary);
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        if buffer.trim().is_empty() {
            return Ok(())
        }
        println!("Correctness: {}", vocabulary.check(buffer.trim()).unwrap());
        println!("Overall correctness: {}", vocabulary.correct);
        datastore.update_vocab(vocabulary).unwrap();
    }
}

/*
#[test]
fn test_adding_new_vocab_in_main_rs() {
    let mut datastore = SqlLiteDatastore::create().unwrap();
    datastore.add_vocab("Tree", "Baum").is_ok();
}
*/
