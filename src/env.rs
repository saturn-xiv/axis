use std::collections::HashMap;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use chrono::Utc;
use handlebars::Handlebars;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use super::{
    errors::Result,
    shell::{Auth, Command, Local, Ssh},
    ROOT,
};

include!(concat!(env!("OUT_DIR"), "/env.rs"));

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &str = include_str!("banner.txt");

pub type Vars = HashMap<String, String>;

pub const EXT: &str = "toml";

pub fn parse<P: AsRef<Path>, T: DeserializeOwned>(file: P) -> Result<T> {
    let mut file = File::open(file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let it = toml::from_slice(&buf)?;
    Ok(it)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    name: String,
    groups: Vec<String>,
    hosts: Vec<String>,
    tasks: Vec<Task>,
    vars: Vars,
}

impl Job {
    pub fn load(name: &str) -> Result<Vec<Self>> {
        let file = Path::new("jobs").join(name).with_extension(EXT);
        info!("load jobs from {}", file.display());

        let mut global = Vars::new();
        {
            global.insert(
                "timestamp".to_string(),
                Utc::now().format("%y%m%d%H%M%S%3f").to_string(),
            );
            global.insert("uuid".to_string(), Uuid::new_v4().to_string());
            {
                let mut rng = thread_rng();
                global.insert(
                    "random".to_string(),
                    std::iter::repeat(())
                        .map(|()| rng.sample(Alphanumeric))
                        .take(32)
                        .collect(),
                );
            }
        }

        let mut items: Vec<Self> = parse(file)?;
        for it in &mut items {
            it.vars.extend(global.clone());
        }
        Ok(items)
    }
    pub fn run(&self, inventory: &str) -> Result<()> {
        info!("run job {} under inventory {}", self.name, inventory);
        let mut hosts = Vec::new();
        for it in &self.groups {
            hosts.extend(Group::load(inventory, &it, self.vars.clone())?);
        }
        for it in &self.hosts {
            hosts.push(Host::load(inventory, &it, self.vars.clone())?);
        }
        for (host, vars) in &hosts {
            let host = host.clone();
            let vars = vars.clone();
            let tasks = self.tasks.clone();
            std::thread::spawn(move || {
                if let Err(e) = Self::handle(&host, &vars, &tasks) {
                    error!("host {}: {:?}", host, e);
                }
            });
        }
        Ok(())
    }

    fn handle(hostname: &str, vars: &Vars, tasks: &[Task]) -> Result<()> {
        debug!("host {}:\n{:?}", hostname, vars);
        if hostname == "localhost" || hostname == "127.0.0.1" {
            let host = Local;
            for task in tasks {
                Host::run(&host, task, vars)?;
            }
        } else {
            let host = Ssh::new(
                hostname,
                match vars.get("ssh_port") {
                    Some(v) => Some(v.parse()?),
                    None => None,
                },
                match vars.get("ssh_user") {
                    Some(v) => Some(v.to_string()),
                    None => None,
                },
                if let Some(v) = vars.get("ssh_password") {
                    Some(Auth::Password(v.to_string()))
                } else if let Some(v) = vars.get("ssh_private_key") {
                    Some(Auth::Key(Path::new(v).to_path_buf()))
                } else {
                    None
                },
            )?;

            for task in tasks {
                Host::run(&host, task, vars)?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    hosts: Vec<String>,
    vars: Vars,
}

impl Group {
    pub fn load(inventory: &str, name: &str, parent: Vars) -> Result<Vec<(String, Vars)>> {
        let file = Path::new(inventory)
            .join("groups")
            .join(name)
            .with_extension(EXT);
        info!("load group from {}", file.display());
        let it: Self = parse(file)?;

        let mut group = Vars::new();
        group.extend(parent);
        group.extend(it.vars.clone());

        let mut items = Vec::new();
        for host in &it.hosts {
            items.push(Host::load(inventory, &host, group.clone())?);
        }
        Ok(items)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Host;

impl Host {
    pub fn load(inventory: &str, name: &str, parent: Vars) -> Result<(String, Vars)> {
        let file = Path::new(inventory)
            .join("hosts")
            .join(name)
            .with_extension(EXT);
        let mut vars = Vars::new();
        vars.extend(parent);
        if file.exists() {
            info!("load host from {}", file.display());
            let cur: Vars = parse(file)?;
            vars.extend(cur);
        }
        Ok((name.to_string(), vars))
    }
    pub fn run<T: Command + fmt::Display>(host: &T, task: &Task, vars: &Vars) -> Result<()> {
        info!("run {} on {}", task, host);
        match task {
            Task::Upload {
                remote,
                local,
                owner,
                group,
                mode,
            } => {
                host.upload(local, remote)?;
                if let Some(owner) = owner {
                    Self::run(host, &Task::chown(remote, owner, None), vars)?;
                }
                if let Some(group) = group {
                    Self::run(host, &Task::chgrp(remote, group, None), vars)?;
                }
                if let Some(mode) = mode {
                    Self::run(host, &Task::chmod(remote, mode, None), vars)?;
                }
            }
            Task::Download {
                remote,
                local,
                owner,
                group,
                mode,
            } => {
                let root = ROOT.join(host.to_string()).join("downloads");
                if !root.exists() {
                    create_dir_all(&root)?;
                }
                let local = root.join(local);
                host.download(remote, &local)?;
                if cfg!(unix) {
                    if let Some(owner) = owner {
                        Self::run(&Local, &Task::chown(&local, owner, None), vars)?;
                    }
                    if let Some(group) = group {
                        Self::run(&Local, &Task::chgrp(&local, group, None), vars)?;
                    }
                    if let Some(mode) = mode {
                        Self::run(&Local, &Task::chmod(&local, mode, None), vars)?;
                    }
                }
            }
            Task::Template {
                remote,
                local,
                owner,
                group,
                mode,
            } => {
                let tmp = Task::template(&local, vars)?;
                Self::run(
                    host,
                    &Task::Upload {
                        remote: remote.clone(),
                        local: tmp,
                        owner: owner.clone(),
                        group: group.clone(),
                        mode: mode.clone(),
                    },
                    vars,
                )?;
            }
            Task::Shell { user, script } => {
                host.script(user.clone(), &script)?;
            }
        };
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Task {
    Upload {
        remote: PathBuf,
        local: PathBuf,
        owner: Option<String>,
        group: Option<String>,
        mode: Option<String>,
    },
    Download {
        remote: PathBuf,
        local: PathBuf,
        owner: Option<String>,
        group: Option<String>,
        mode: Option<String>,
    },
    Template {
        remote: PathBuf,
        local: PathBuf,
        owner: Option<String>,
        group: Option<String>,
        mode: Option<String>,
    },
    Shell {
        user: Option<String>,
        script: String,
    },
}

impl Task {
    fn chgrp<P: AsRef<Path>>(file: P, group: &str, user: Option<String>) -> Self {
        let file = file.as_ref().display();
        Self::Shell {
            script: format!("chgrp {} {}", group, file),
            user,
        }
    }
    fn chown<P: AsRef<Path>>(file: P, owner: &str, user: Option<String>) -> Self {
        let file = file.as_ref().display();
        Self::Shell {
            script: format!("chown {} {}", owner, file),
            user,
        }
    }
    fn chmod<P: AsRef<Path>>(file: P, mode: &str, user: Option<String>) -> Self {
        let file = file.as_ref().display();
        Self::Shell {
            script: format!("chmod {} {}", mode, file),
            user,
        }
    }
    fn template<P: AsRef<Path>>(tpl: P, var: &Vars) -> Result<PathBuf> {
        let root = ROOT.join("cache");
        if !root.exists() {
            create_dir_all(&root)?;
        }
        let rdr = root.join(Uuid::new_v4().to_string());
        {
            let rdr = File::create(&rdr)?;
            let tpl = tpl.as_ref();
            let name = format!("{}", tpl.display());
            let mut reg = Handlebars::new();
            reg.set_strict_mode(true);
            reg.register_template_file(&name, tpl)?;
            reg.render_to_write(&name, var, &rdr)?;
        }
        Ok(rdr)
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Upload {
                remote,
                local,
                owner: _,
                group: _,
                mode: _,
            } => write!(f, "upload {} to {}", local.display(), remote.display()),
            Self::Download {
                remote,
                local,
                owner: _,
                group: _,
                mode: _,
            } => write!(f, "download {} to {}", remote.display(), local.display()),
            Self::Template {
                remote,
                local,
                owner: _,
                group: _,
                mode: _,
            } => write!(
                f,
                "render template {} to {}",
                local.display(),
                remote.display()
            ),
            Self::Shell { script, user } => write!(
                f,
                "run shell script {}{}",
                script,
                match user {
                    Some(v) => format!(" as user {}", v),
                    None => "".to_string(),
                }
            ),
        }
    }
}
