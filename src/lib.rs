#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;

extern crate chrono;
extern crate clap;
extern crate dirs;
extern crate rand;
extern crate ssh2;
extern crate toml;
extern crate uuid;

pub mod app;
pub mod controllers;
pub mod env;
pub mod errors;
pub mod models;
pub mod orm;
pub mod shell;

use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref ROOT: PathBuf = Path::new("tmp").to_path_buf();
}
