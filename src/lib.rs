#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

extern crate base64;
extern crate chrono;
extern crate clap;
extern crate log4rs;
extern crate mustache;
extern crate nix;
extern crate rmp_serde;
extern crate serde;
extern crate toml;
extern crate uuid;
extern crate zmq;

pub mod agent;
pub mod errors;
pub mod key;
pub mod master;
pub mod orm;
pub mod protocol;
pub mod task;

use std::default::Default;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::{App, Arg, SubCommand};
use diesel::Connection as DieselConnection;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug)]
pub struct Port(pub u16);

impl Port {
    pub fn publisher(&self) -> u16 {
        self.0
    }
    pub fn reporter(&self) -> u16 {
        self.0 + 1
    }
}

pub fn parse<P: AsRef<Path>, T: DeserializeOwned>(file: P) -> errors::Result<T> {
    let mut file = File::open(file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let it = toml::from_slice(&buf)?;
    Ok(it)
}

pub fn launch() -> errors::Result<()> {
    let name = env!("CARGO_PKG_NAME");
    let master = "master";
    let agent = "agent";
    let mut etc = Path::new("/etc").join(name);
    if !etc.exists() {
        etc = Path::new(".etc").to_path_buf();
    }
    log4rs::init_file(etc.join("log4rs.yml"), Default::default())?;

    let matches = App::new(name)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .before_help(include_str!("banner.txt"))
        .after_help(env!("CARGO_PKG_HOMEPAGE"))
        .subcommand(
            SubCommand::with_name(master)
                .about("The master, used to control the agents")
                .arg(
                    Arg::with_name("list")
                        .short("L")
                        .long("list")
                        .help("List all agents"),
                )
                .arg(
                    Arg::with_name("accept")
                        .short("A")
                        .long("accept")
                        .takes_value(true)
                        .help("Accept the specified agent"),
                )
                .arg(
                    Arg::with_name("reject")
                        .long("reject")
                        .short("R")
                        .takes_value(true)
                        .help("Reject the specified agent"),
                )
                .arg(
                    Arg::with_name("finger")
                        .long("finger")
                        .short("F")
                        .help("Prints all fingerprints"),
                ),
        )
        .subcommand(
            SubCommand::with_name(agent)
                .about("The agent, receives commands from a remote master")
                .arg(
                    Arg::with_name("finger")
                        .long("finger")
                        .short("F")
                        .help("Prints all fingerprints"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches(agent) {
        if matches.is_present("finger") {
            return agent::finger(etc);
        }
        return agent::launch(etc);
    }

    let db = orm::Connection::establish("tmp/db")?;

    if let Some(matches) = matches.subcommand_matches(master) {
        if matches.is_present("list") {
            return master::agents::list(etc, db);
        }
        if matches.is_present("accept") {
            let name = matches.value_of("accept").unwrap();
            return master::agents::accept(etc, db, name);
        }
        if matches.is_present("reject") {
            let name = matches.value_of("reject").unwrap();
            return master::agents::reject(etc, db, name);
        }
        if matches.is_present("finger") {
            return master::finger(etc, db);
        }
        return master::launch(etc, db);
    }
    Ok(())
}
