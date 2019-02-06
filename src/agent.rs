use std::path::PathBuf;

use super::errors::Result;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub id: String,
    pub master: Master,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Master {
    pub port: u16,
    pub host: String,
    pub finger: String,
}

pub fn launch(_cfg: PathBuf) -> Result<()> {
    Ok(())
}

pub fn finger(_cfg: PathBuf) -> Result<()> {
    Ok(())
}
