pub mod models;
pub mod schema;

use std::path::Path;

use diesel::{
    connection::SimpleConnection, sqlite::SqliteConnection, Connection as DieselConnection,
};
use failure::Error;

use super::errors::Result;

#[database("sqlite")]
pub struct Database(SqliteConnection);

pub type Connection = SqliteConnection;

pub fn open<T: AsRef<str>>(db: T) -> Result<Connection> {
    let db = db.as_ref();
    let exists = Path::new(db).exists();
    let db = Connection::establish(db)?;

    if !exists {
        info!("init database tables");
        let script = include_str!("up.sql");
        db.transaction::<(), Error, _>(|| {
            info!("run sql\n{}", script);
            db.batch_execute(script)?;
            Ok(())
        })?;
    }
    Ok(db)
}
