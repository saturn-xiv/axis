use std::collections::HashMap;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use chrono::{NaiveDateTime, Utc};
use failure::Error;
use handlebars::Handlebars;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use super::{
    errors::Result,
    shell::{Auth, Command, Local, Ssh},
    ROOT,
};

pub type Vars = HashMap<String, String>;
pub type Excutor = (String, HashMap<String, Vars>, Vec<Task>);

pub const EXT: &str = "toml";
pub const ALL: &str = "all";

macro_rules! load_host_vars {
    ($i:expr, $n:expr, $v:expr) => {
        let file = Path::new($i).join("hosts").join($n).with_extension(EXT);
        if file.exists() {
            debug!("load vars from {}", file.display());
            let cur: Vars = parse(file)?;
            $v.extend(cur);
        }
    };
}

#[derive(Debug)]
pub struct Report {
    pub job: String,
    pub updated: NaiveDateTime,
    pub reason: Option<Error>,
}

impl Report {
    pub fn new(job: String, reason: Option<Error>) -> Self {
        Self {
            job,
            reason,
            updated: Utc::now().naive_local(),
        }
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:32} {:16} {}",
            self.job,
            self.updated,
            match &self.reason {
                Some(e) => {
                    e.to_string()
                }
                None => "Done.".to_string(),
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub name: String,
    pub groups: Vec<String>,
    pub hosts: Vec<String>,
    pub tasks: Vec<Task>,
    pub vars: Vars,
}

impl Job {
    pub fn load(inventory: &str, name: &str) -> Result<Vec<Excutor>> {
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
        let mut items = Vec::new();
        let file = Path::new("jobs").join(name).with_extension(EXT);
        debug!("load jobs from {}", file.display());
        let jobs: Vec<Job> = parse(file)?;
        for job in &jobs {
            let mut vars = Vars::new();
            vars.extend(global.clone());
            vars.extend(job.vars.clone());
            let mut hosts = HashMap::new();
            for group in &job.groups {
                hosts.extend(Group::load(inventory, group, vars.clone())?);
            }
            for host in &job.hosts {
                hosts.insert(host.to_string(), Host::load(inventory, host, vars.clone())?);
            }

            items.push((job.name.clone(), hosts, job.tasks.clone()));
        }
        Ok(items)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub hosts: Vec<String>,
    pub vars: Vars,
}

impl Group {
    pub fn load(inventory: &str, name: &str, parent: Vars) -> Result<HashMap<String, Vars>> {
        let mut items = HashMap::new();

        let mut vars = Vars::new();
        vars.extend(parent);
        load_host_vars!(inventory, ALL, vars);

        let file = Path::new(inventory)
            .join("groups")
            .join(name)
            .with_extension(EXT);
        debug!("load vars from {}", file.display());
        let group: Self = parse(file)?;
        vars.extend(group.vars);

        for host in &group.hosts {
            let mut cur = Vars::new();
            cur.extend(vars.clone());
            load_host_vars!(inventory, name, cur);
            items.insert(host.to_string(), cur);
        }
        Ok(items)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Host;

impl Host {
    pub fn load(inventory: &str, name: &str, parent: Vars) -> Result<Vars> {
        let mut vars = Vars::new();
        vars.extend(parent);
        load_host_vars!(inventory, ALL, vars);
        load_host_vars!(inventory, name, vars);
        Ok(vars)
    }

    pub fn handle(hostname: &str, vars: &Vars, tasks: &[Task]) -> Result<()> {
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

    fn run<T: Command + fmt::Display>(host: &T, task: &Task, vars: &Vars) -> Result<()> {
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
#[serde(rename_all = "camelCase", tag = "type")]
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

fn parse<P: AsRef<Path>, T: DeserializeOwned>(file: P) -> Result<T> {
    let mut file = File::open(file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let it = toml::from_slice(&buf)?;
    Ok(it)
}
