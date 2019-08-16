#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

extern crate base64;
extern crate chrono;
extern crate clap;
extern crate log4rs;
extern crate nix;
extern crate serde;
extern crate serde_json;
extern crate tera;
extern crate toml;
extern crate uuid;

pub mod agent;
pub mod errors;
pub mod launcher;
pub mod master;
pub mod orm;

use std::path::Path;

use clap::App;

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn launch() -> errors::Result<()> {
    let etc = {
        let mut etc = Path::new("/etc").join(NAME);
        if !etc.exists() {
            etc = Path::new(".etc").to_path_buf();
        }
        etc
    };

    let var = {
        let mut var = Path::new("/var").join("lib").join(NAME);
        if !var.exists() {
            var = Path::new(".var").to_path_buf();
        }
        var
    };

    log4rs::init_file(etc.join("log4rs.yml"), Default::default())?;

    App::new(NAME)
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .before_help(include_str!("banner.txt"))
        .after_help(env!("CARGO_PKG_HOMEPAGE"))
        .get_matches();

    launcher::start(etc, var)
}
