mod models;
mod schema;

use std::fs::create_dir_all;
use std::ops::Deref;
use std::path::Path;

use diesel::{connection::SimpleConnection, r2d2::ConnectionManager, SqliteConnection};

use super::{errors::Result, ROOT};

pub type Connection = SqliteConnection;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<Connection>>;
pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager<Connection>>;

const UP: &str = include_str!("up.sql");

pub fn open() -> Result<Pool> {
    let db = Path::new("tmp").join("db");
    let url = db.display().to_string();
    debug!("load db from {}", url);
    if !ROOT.exists() {
        create_dir_all(ROOT.deref())?;
    }
    if !db.exists() {
        let db = Pool::new(ConnectionManager::<Connection>::new(&url))?;
        let db = db.get()?;
        db.batch_execute(UP)?;
    }
    let db = Pool::new(ConnectionManager::<Connection>::new(&url))?;
    {
        let db = db.get()?;
        db.batch_execute(&format!(
            "PRAGMA foreign_keys = ON; PRAGMA busy_timeout = {};",
            5000
        ))?;
    }
    Ok(db)
}
