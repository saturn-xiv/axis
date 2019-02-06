use std::path::PathBuf;

use super::super::{errors::Result, orm::Connection};

pub fn list(_cfg: PathBuf, _db: Connection) -> Result<()> {
    Ok(())
}

pub fn accept(_cfg: PathBuf, _db: Connection, _name: &str) -> Result<()> {
    Ok(())
}

pub fn reject(_cfg: PathBuf, _db: Connection, _name: &str) -> Result<()> {
    Ok(())
}
