use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use uuid::Uuid;

use super::super::errors::Result;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Uuid,
    pub payload: Vec<Payload>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=== begin task {} ===\n", self.id)?;
        for it in self.payload.iter() {
            write!(f, "------")?;
            write!(f, "{}\n", it)?;
            write!(f, "------")?;
        }
        write!(f, "=== end task ===")?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Payload {
    Shell { user: String, script: String },
    File { path: String, body: Vec<u8> },
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Payload::Shell { user, script } => write!(f, "shell {}\n{}", user, script),
            Payload::File { path, body } => write!(f, "file {}, {} bytes", path, body.len()),
        }
    }
}
impl Payload {
    pub fn execute(&self) -> Result<String> {
        match self {
            Payload::Shell { user, script } => {
                info!("run as {}\n{}", user, script);
                let output = Command::new("su")
                    .arg("-")
                    .arg(user)
                    .arg("-c")
                    .arg(script)
                    .output()?;
                Ok(format!("{:?}", output))
            }
            Payload::File { path, body } => {
                info!("write to file {}", path);
                if let Some(d) = Path::new(path).parent() {
                    if !d.exists() {
                        info!("{} is not exist, will create it", d.display());
                        create_dir_all(&d)?;
                    }
                }
                let mut file = File::open(path)?;
                file.write_all(&body)?;
                Ok("OK".to_string())
            }
        }
    }
}
