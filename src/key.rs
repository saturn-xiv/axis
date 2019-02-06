use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use zmq::CurveKeyPair;

use super::errors::Result;

pub type KEY = [u8; 32];

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub public: KEY,
    pub private: KEY,
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pub: {}\npri: {}",
            base64::encode(&self.public),
            base64::encode(&self.private),
        )
    }
}

impl Pair {
    pub fn new(etc: &PathBuf) -> Result<Self> {
        let file = etc.join("key");
        if file.exists() {
            let it = rmp_serde::decode::from_read(File::open(file)?)?;
            return Ok(it);
        }

        let pair = CurveKeyPair::new()?;
        let it = Self {
            public: pair.public_key,
            private: pair.secret_key,
        };

        let mut fd = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o400)
            .open(file)?;
        fd.write_all(&rmp_serde::encode::to_vec(&it)?)?;

        Ok(it)
    }
}
