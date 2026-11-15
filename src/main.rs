use datastore::{Datastore, SqlLiteDatastore};
use rusqlite::Result;

mod datastore;
mod error;

fn main() -> Result<()> {
    let mut datastore = SqlLiteDatastore::create().unwrap();
    // do stuff
    datastore.add_vocab("Street", "Strasse").unwrap();
    let a = datastore
        .check("Street".to_string(), "strase".to_string())
        .unwrap();
    println!("{}", a);

    Ok(())
}
//
//
//

/*
#[test]
fn test_adding_new_vocab_in_main_rs() {
    let mut datastore = SqlLiteDatastore::create().unwrap();
    datastore.add_vocab("Tree", "Baum").is_ok();
}
*/
