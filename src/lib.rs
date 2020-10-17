#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate clap;
extern crate rand;
extern crate toml;
extern crate uuid;

pub mod app;
pub mod env;
pub mod errors;
pub mod models;

use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref ROOT: PathBuf = Path::new("tmp").to_path_buf();
}
