use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use serde_json::Value;

use super::errors::Result;

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &'static str = include_str!("banner.txt");

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub hosts: Vec<Host>,
    pub environment: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Task {
    Ping,
    Shell((String, String)),
    File((String, Vec<u8>)),
}

impl Task {
    pub fn run(self) -> Result<Value> {
        match self {
            Task::Ping => {
                info!("ping");
                let mut os_release = String::new();
                let mut file = File::open("/etc/os-release")?;
                file.read_to_string(&mut os_release)?;
                let un = nix::sys::utsname::uname();
                let si = nix::sys::sysinfo::sysinfo()?;
                Ok(json! ({
                    "sysname": un.sysname(),
                    "nodename": un.nodename(),
                    "release": un.release(),
                    "version": un.version(),
                    "machine": un.machine(),
                    "uptime": si.uptime(),
                    "load average": si.load_average(),
                    "os-release": os_release,
                }))
            }
            Task::Shell((user, script)) => {
                info!("run as {}\n{}", user, script);
                let output = Command::new("su")
                    .arg("-")
                    .arg(user)
                    .arg("-c")
                    .arg(script)
                    .output()?;
                Ok(json!(format!("{:?}", output)))
            }
            Task::File((name, body)) => {
                info!("write to file {}", name);
                let mut file = File::open(name)?;
                file.write_all(&body)?;
                Ok(json!({}))
            }
        }
    }
}
