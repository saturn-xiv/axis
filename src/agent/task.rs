use std::fmt;
use std::fs::{create_dir_all, OpenOptions};
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
        write!(f, "==={}===\n", self.id)?;
        for it in self.payload.iter() {
            write!(f, "------\n")?;
            write!(f, "{}\n", it)?;
            write!(f, "------\n")?;
        }
        write!(f, "======")?;
        Ok(())
    }
}

impl Task {
    pub fn execute(&self) -> Result<String> {
        let mut buf = String::new();
        for it in self.payload.iter() {
            if let Some(it) = it.execute()? {
                buf += "\n";
                buf += &it;
            }
        }
        Ok(buf)
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
    pub fn execute(&self) -> Result<Option<String>> {
        match self {
            Payload::Shell { user, script } => {
                info!("run as {}\n{}", user, script);
                let output = Command::new("su")
                    .arg("-")
                    .arg(user)
                    .arg("-c")
                    .arg(script)
                    .output()?;

                Ok(Some(format!(
                    "status: {} \nstdout: {} \nstderr: {}",
                    output.status,
                    String::from_utf8_lossy(output.stdout.as_slice()),
                    String::from_utf8_lossy(output.stderr.as_slice()),
                )))
            }
            Payload::File { path, body } => {
                info!("write to file {}", path);
                if let Some(d) = Path::new(path).parent() {
                    if !d.exists() {
                        info!("{} is not exist, will create it", d.display());
                        create_dir_all(&d)?;
                    }
                }
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)?;
                file.write_all(&body)?;
                Ok(None)
            }
        }
    }
}
