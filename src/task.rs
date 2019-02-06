use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use uuid::Uuid;

use super::errors::Result;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Uuid,
    pub payload: Payload,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "task {}\n{}", self.id, self.payload)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Payload {
    Shell((String, String)),
    File((String, Vec<u8>)),
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Payload::Shell((ref user, ref script)) => write!(f, "shell {}\n{}", user, script),
            Payload::File((ref name, ref body)) => write!(f, "file {}, {} bytes", name, body.len()),
        }
    }
}
impl Payload {
    pub fn execute(self) -> Result<String> {
        match self {
            Payload::Shell((user, script)) => {
                info!("run as {}\n{}", user, script);
                let output = Command::new("su")
                    .arg("-")
                    .arg(user)
                    .arg("-c")
                    .arg(script)
                    .output()?;
                Ok(format!("{:?}", output))
            }
            Payload::File((name, body)) => {
                info!("write to file {}", name);
                let mut file = File::open(name)?;
                file.write_all(&body)?;
                Ok("OK".to_string())
            }
        }
    }
}
