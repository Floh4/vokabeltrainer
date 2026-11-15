use datastore::{Datastore, SqlLiteDatastore};
use rusqlite::Result;

mod datastore;
mod error;

fn main() -> Result<()> {
    let datastore = SqlLiteDatastore::create().unwrap();
    // do stuff

    Ok(())
}

/*
#[test]
fn test_adding_new_vocab_in_main_rs() {
    let mut datastore = SqlLiteDatastore::create().unwrap();
    datastore.add_vocab("Tree", "Baum").is_ok();
}
*/
