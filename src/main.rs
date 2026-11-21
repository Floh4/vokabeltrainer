use crate::vocabulary::add;
use datastore::{Datastore, SqlLiteDatastore};
use rusqlite::Result;
use std::io;

mod datastore;
mod error;
mod vocabulary;

fn main() -> Result<()> {
    let mut datastore = SqlLiteDatastore::create_in_memory().unwrap();
    // do stuff
    datastore.add_vocab(add("Street", "Strasse")).unwrap();
    datastore.add_vocab(add("Neighbour", "Nachbar")).unwrap();
    datastore.add_vocab(add("Cat", "Katze")).unwrap();
    loop {
        let mut vocabulary = datastore
            .id_to_vocabulary(datastore.random_id().unwrap())
            .unwrap();
        println!("Enter translation for {}", vocabulary.vocabulary);
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        if buffer.trim().is_empty() {
            return Ok(());
        }
        println!("Correctness: {}", vocabulary.check(buffer.trim()).unwrap());
        println!("Overall correctness: {}", vocabulary.correct);
        datastore.update_vocab(vocabulary).unwrap();
    }
}

// TODO:
// - BUGFIX: When corectness / count == 1 then the vocab is never selected -> when all entrys are
// like that -> infinite loop because it will never select anything and keeps trying to select
// something
//
// - random_id function will never ask the vocab again if corectness and count are both 1 ->
// make it to 0.001 or something
//
// - maybe add last_vocab parameter to random_id so that the same id isnt selected twice
//
// - add a function to add vocabulary directly(what add_vocab did before it took vocabulary as
// argument)
//
// - add function to add information
