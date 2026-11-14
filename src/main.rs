use datastore::SqlLiteDatastore;
use rusqlite::Result;

mod error;
mod datastore;

fn main() -> Result<()> {
    let datastore = SqlLiteDatastore::create();
    // do stuff

    Ok(())
}

