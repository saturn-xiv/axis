#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

extern crate base64;
extern crate chrono;
extern crate clap;
extern crate failure;
extern crate log4rs;
extern crate nix;
extern crate rmp_serde;
extern crate serde;
extern crate serde_json;
extern crate tera;
extern crate toml;
extern crate uuid;
extern crate zmq;

pub mod agent;
pub mod errors;
pub mod key;
pub mod master;
pub mod orm;
pub mod protocol;
pub mod publish;

use std::default::Default;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use clap::{App, Arg, SubCommand};
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

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn launch() -> errors::Result<()> {
    let master = "master";
    let agent = "agent";
    let publish = "publish";
    let db = "tmp/db";

    let etc = {
        let mut d = Path::new("/etc").join(NAME);
        if !d.exists() {
            d = Path::new(".etc").to_path_buf();
        }
        d
    };

    log4rs::init_file(etc.join("log4rs.yml"), Default::default())?;

    let matches = App::new(NAME)
        .version(VERSION)
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
                    Arg::with_name("delete")
                        .long("delete")
                        .short("D")
                        .takes_value(true)
                        .help("Delete the specified agent"),
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
        .subcommand(
            SubCommand::with_name(publish)
                .about("Publish task to agents")
                .arg(
                    Arg::with_name("group")
                        .short("G")
                        .long("group")
                        .takes_value(true)
                        .required(true)
                        .help("Publish by group"),
                )
                .arg(
                    Arg::with_name("task")
                        .long("task")
                        .short("T")
                        .takes_value(true)
                        .required(true)
                        .help("Task name"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches(agent) {
        if matches.is_present("finger") {
            return finger(etc, agent);
        }
        return agent::launch(etc);
    }

    if let Some(matches) = matches.subcommand_matches(master) {
        let db = orm::open(db)?;
        if matches.is_present("list") {
            return master::agents::list(db);
        }
        if matches.is_present("accept") {
            let name = matches.value_of("accept").unwrap();
            return master::agents::accept(db, name);
        }
        if matches.is_present("reject") {
            let name = matches.value_of("reject").unwrap();
            return master::agents::reject(db, name);
        }
        if matches.is_present("delete") {
            let name = matches.value_of("delete").unwrap();
            return master::agents::delete(db, name);
        }
        if matches.is_present("finger") {
            return finger(etc, master);
        }
        return master::launch(etc, db);
    }

    if let Some(matches) = matches.subcommand_matches(publish) {
        let db = orm::open(db)?;
        return publish::launch(
            etc,
            matches.value_of("group").unwrap(),
            matches.value_of("task").unwrap(),
            db,
        );
    }
    Ok(())
}

fn finger(etc: PathBuf, m: &'static str) -> errors::Result<()> {
    let mut file = etc.join(m);
    file.set_extension("key");
    let pair = key::Pair::new(file)?;
    println!("{}", pair);
    Ok(())
}
