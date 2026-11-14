use datastore::SqlLiteDatastore;
use rusqlite::Result;

mod error;
mod datastore;

fn main() -> Result<()> {
    let datastore = SqlLiteDatastore::create();
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
