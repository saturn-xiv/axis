use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::str::FromStr;

use failure::Error;
use zmq::CurveKeyPair;

use super::errors::Result;

pub type KEY = [u8; 32];

#[derive(Serialize, Deserialize)]
pub struct Key(pub [u8; 32]);

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let buf: Vec<u8> = base64::decode(s)?;

        let mut key: KEY = Default::default();
        let len = key.len();
        key.copy_from_slice(&buf[0..len]);
        Ok(Key(key))
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", base64::encode(&self.0))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub public: Key,
    pub private: Key,
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pub: {}\npri: {}", self.public, self.private,)
    }
}

impl Pair {
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self> {
        let file = file.as_ref();
        if file.exists() {
            let it = rmp_serde::decode::from_read(File::open(file)?)?;
            return Ok(it);
        }

        let pair = CurveKeyPair::new()?;
        let it = Self {
            public: Key(pair.public_key),
            private: Key(pair.secret_key),
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
