pub mod agents;

use std::collections::HashMap;
use std::path::PathBuf;

use super::{errors::Result, orm::Connection};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub port: u16,
    pub tasks: Vec<Group>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub hosts: Vec<Host>,
    pub environment: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: String,
}

pub fn launch(_etc: PathBuf, _db: Connection) -> Result<()> {
    Ok(())
}

pub fn finger(_etc: PathBuf, _db: Connection) -> Result<()> {
    Ok(())
}
