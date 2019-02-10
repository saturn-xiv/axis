pub mod agents;
pub mod config;

use std::path::PathBuf;

use super::{errors::Result, orm::Connection};

pub fn launch(_etc: PathBuf, _db: Connection) -> Result<()> {
    Ok(())
}
