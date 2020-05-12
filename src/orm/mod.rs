pub mod models;
pub mod schema;

use std::path::Path;
use std::time::Duration;

use diesel::{
    connection::{Connection as DieselConnection, SimpleConnection},
    SqliteConnection,
};

use super::errors::Result;

pub type Connection = SqliteConnection;
pub type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Connection>>;
pub type PooledConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<Connection>>;

pub fn open() -> Result<Pool> {
    let file = Path::new("tmp").join("db");
    let url = file.display().to_string();
    if !file.exists() {
        warn!("db not exists, will create it.");
        let db = Connection::establish(&url)?;
        db.batch_execute(include_str!("up.sql"))?;
    }
    let db = Pool::new(diesel::r2d2::ConnectionManager::<Connection>::new(&url))?;
    {
        let db = db.get()?;
        db.batch_execute(&format!(
            "PRAGMA synchronous = OFF; PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON; PRAGMA busy_timeout = {};",
            Duration::from_secs(1<<4).as_micros()
        ))?;
    }
    Ok(db)
}
