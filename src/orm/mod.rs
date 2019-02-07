pub mod schema;

use diesel::Connection as DieselConnection;

use super::errors::Result;

pub type Connection = diesel::sqlite::SqliteConnection;

pub fn open<T: AsRef<str>>(db: T) -> Result<Connection> {
    let it = Connection::establish(db.as_ref())?;
    Ok(it)
}
