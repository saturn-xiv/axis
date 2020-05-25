use std::collections::BTreeMap;
use std::fmt;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{prelude::*, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Command as ShellCommand, Stdio};

use chrono::Utc;
use handlebars::Handlebars;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use super::errors::Result;

pub const CONFIG_EXT: &str = "toml";
pub const TEMPLATE_EXT: &str = "hbs";

pub type Vars = BTreeMap<String, String>;
pub type Excutor = (Vec<Host>, Vec<Command>);
pub type Host = (String, Vars);

macro_rules! load_vars {
    ($i:expr, $n:expr, $v:expr) => {
        let file = $i.join(&format!("{}.{}", $n, CONFIG_EXT));
        if file.exists() {
            debug!("load vars from {}", file.display());
            let cur: Vars = parse(file)?;
            $v.extend(cur);
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub hosts: Vec<String>,
    pub vars: Vars,
}

impl Group {
    pub fn load(name: &str, inventory: &str, parent: Vars) -> Result<Vec<Host>> {
        info!("load group {}@{}", name, inventory);
        let group = {
            let mut it: Self = parse(
                Path::new(inventory)
                    .join("groups")
                    .join(name)
                    .with_extension(CONFIG_EXT),
            )?;
            it.vars = {
                let mut vars = Vars::new();
                vars.extend(parent);
                vars.extend(it.vars);
                vars
            };
            it
        };

        let mut items = Vec::new();
        for host in group.hosts.iter() {
            let mut vars = Vars::new();
            vars.extend(group.vars.clone());
            load_vars!(Path::new(inventory).join("hosts"), host, vars);
            items.push((host.clone(), vars));
        }
        Ok(items)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub vars: Vars,
    pub tasks: Vec<Task>,
}

impl Job {
    pub fn load(name: &str, inventory: &str) -> Result<Vec<Excutor>> {
        info!("load job {}@{}", name, inventory);
        let job = {
            let mut it: Self = parse(&Path::new("jobs").join(name).with_extension(CONFIG_EXT))?;
            it.vars.insert(
                "timestamp".to_string(),
                Utc::now().format("%y%m%d%H%M%S%3f").to_string(),
            );
            it.vars
                .insert("uuid".to_string(), Uuid::new_v4().to_string());
            {
                let mut rng = thread_rng();
                it.vars.insert(
                    "random".to_string(),
                    std::iter::repeat(())
                        .map(|()| rng.sample(Alphanumeric))
                        .take(32)
                        .collect(),
                );
            }
            it
        };
        let mut excutors = Vec::new();
        for task in job.tasks.iter() {
            info!("load task {}@{}", task.name, inventory);
            for group in task.groups.iter() {
                let mut vars = Vars::new();
                vars.extend(job.vars.clone());
                let hosts = Group::load(group, inventory, vars)?;
                excutors.push((hosts, task.commands.clone()));
            }
        }
        Ok(excutors)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub name: String,
    pub groups: Vec<String>,
    pub commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Command {
    Upload { src: String, dest: String },
    Download { src: String, dest: String },
    Shell { script: String },
}

impl Command {
    const LOCALHOST: &'static str = "localhost";
    pub fn run(&self, host: &str, vars: &Vars) -> Result<()> {
        debug!("host {} env: {:?}", host, vars);
        let user = match vars.get("ssh.user") {
            Some(v) => v.clone(),
            None => "root".to_string(),
        };
        let port: u16 = match vars.get("ssh.port") {
            Some(v) => v.parse()?,
            None => 22,
        };
        let key: String = match vars.get("ssh.key-file") {
            Some(v) => v.clone(),
            None => "~/.ssh/id_rsa".to_string(),
        };
        match self {
            Self::Upload { src, dest } => {
                let src = template(src, vars)?;
                if host == Self::LOCALHOST {
                    shell(
                        host,
                        &format!("rsync -av {src} {dest}", src = src.display(), dest = dest),
                    )?;
                } else {
                    shell(
                        host,
                        &format!(
                            "rsync -azv -e 'ssh -p {port} -i {key}' {src} {user}@{host}:{dest}",
                            src = src.display(),
                            dest = dest,
                            user = user,
                            key = key,
                            host = host,
                            port = port,
                        ),
                    )?;
                }
            }
            Self::Download { src, dest } => {
                let dest = Path::new("tmp").join("downloads").join(host).join(dest);
                if host == Self::LOCALHOST {
                    shell(
                        host,
                        &format!("rsync -av {src} {dest}", src = src, dest = dest.display()),
                    )?;
                } else {
                    shell(
                        host,
                        &format!(
                            "rsync -azv -e 'ssh -p {port} -i {key}' {user}@{host}:{src} {dest}",
                            src = src,
                            dest = dest.display(),
                            user = user,
                            key = key,
                            host = host,
                            port = port,
                        ),
                    )?;
                }
            }
            Self::Shell { script } => {
                let script = template(script, vars)?;
                if host == Self::LOCALHOST {
                    shell(
                        host,
                        &format!("bash -s < {script}", script = script.display()),
                    )?;
                } else {
                    shell(
                        host,
                        &format!(
                            "ssh -p {port} -i {key} {user}@{host} 'bash -s' < {script}",
                            user = user,
                            key = key,
                            host = host,
                            port = port,
                            script = script.display()
                        ),
                    )?;
                }
            }
        };
        Ok(())
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Upload { src, dest } => write!(f, "upload {} to {}", src, dest),
            Self::Download { src, dest } => write!(f, "download {} to {}", src, dest),
            Self::Shell { script } => write!(f, "run shell script {}", script),
        }
    }
}

fn parse<P: AsRef<Path>, T: DeserializeOwned>(file: P) -> Result<T> {
    let file = file.as_ref();
    debug!("load file {}", file.display());
    let mut file = File::open(file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let it = toml::from_slice(&buf)?;
    Ok(it)
}

fn shell(host: &str, script: &str) -> Result<()> {
    info!("actually command: {}", script);
    let root = Path::new("tmp").join("logs");
    if !root.exists() {
        create_dir_all(&root)?;
    }
    let outputs = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(root.join(host).with_extension("log"))?;
    {
        let mut wrt = BufWriter::new(&outputs);
        writeln!(wrt, "{}: {}", Utc::now().naive_local(), script)?;
    }
    let errors = outputs.try_clone()?;

    ShellCommand::new(script)
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;
    Ok(())
}

fn template<P: AsRef<Path>>(tpl: P, vars: &Vars) -> Result<PathBuf> {
    let tpl = tpl.as_ref();
    if tpl.exists() {
        return Ok(tpl.to_path_buf());
    }
    let tpl = tpl.with_extension(TEMPLATE_EXT);
    let root = Path::new("tmp").join("cache");
    if !root.exists() {
        create_dir_all(&root)?;
    }
    let rdr = root.join(Uuid::new_v4().to_string());
    {
        debug!("render {} to {}: {:?}", tpl.display(), rdr.display(), vars);
        let rdr = File::create(&rdr)?;
        let name = tpl.display().to_string();
        let mut reg = Handlebars::new();
        reg.set_strict_mode(true);
        reg.register_template_file(&name, tpl)?;
        reg.render_to_write(&name, vars, &rdr)?;
    }
    Ok(rdr)
}
