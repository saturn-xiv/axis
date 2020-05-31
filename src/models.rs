use std::collections::BTreeMap;
use std::fmt;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{prelude::*, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Command as ShellCommand, Stdio};

use chrono::Utc;
use handlebars::Handlebars;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::de::DeserializeOwned;
use toml::Value;
use uuid::Uuid;

use super::errors::Result;

pub const CONFIG_EXT: &str = "toml";
pub const TEMPLATE_EXT: &str = "hbs";
pub const JOBS: &str = "jobs";

pub type Vars = BTreeMap<String, Value>;
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
    fn git_version() -> Result<String> {
        let out = ShellCommand::new("git")
            .arg("describe")
            .arg("--tags")
            .arg("--always")
            .arg("--dirty")
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| format_err!("could not capture standard output"))?;
        let mut buf = String::new();
        let mut rdr = BufReader::new(out);
        rdr.read_to_string(&mut buf)?;
        Ok(buf.trim().to_string())
    }

    pub fn load(name: &str, inventory: &str) -> Result<Vec<Excutor>> {
        info!("load job {}@{}", name, inventory);
        let job = {
            let mut it: Self = parse(&Path::new(JOBS).join(name).with_extension(CONFIG_EXT))?;
            it.vars
                .insert("job.name".to_string(), Value::String(name.to_string()));
            it.vars.insert(
                "inventory.name".to_string(),
                Value::String(inventory.to_string()),
            );
            it.vars.insert(
                "timestamp".to_string(),
                Value::String(Utc::now().format("%y%m%d%H%M%S%3f").to_string()),
            );
            it.vars.insert(
                "uuid".to_string(),
                Value::String(Uuid::new_v4().to_string()),
            );
            if let Ok(v) = Self::git_version() {
                it.vars.insert("git.version".to_string(), Value::String(v));
            }
            load_vars!(Path::new(inventory), "all", it.vars);
            {
                let mut rng = thread_rng();
                let random: String = std::iter::repeat(())
                    .map(|()| rng.sample(Alphanumeric))
                    .take(32)
                    .collect();
                it.vars.insert("random".to_string(), Value::String(random));
            }
            it
        };
        let mut excutors = Vec::new();
        for task in job.tasks.iter() {
            info!("load task {}@{}", task.name, inventory);
            for group in task.groups.iter() {
                let mut vars = Vars::new();
                vars.extend(job.vars.clone());
                vars.extend(task.vars.clone());
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
    pub vars: Vars,
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

    fn parse_ssh_port(vars: &Vars) -> u16 {
        if let Some(Value::Integer(v)) = vars.get("ssh.port") {
            return *v as u16;
        }
        22
    }
    fn parse_ssh_key_file(inventory: &str, vars: &Vars) -> String {
        if let Some(Value::String(v)) = vars.get("ssh.key-file") {
            return v.clone();
        }
        let key = Path::new(inventory).join("id_rsa");
        if key.exists() {
            key.display().to_string()
        } else {
            "~/.ssh/id_rsa".to_string()
        }
    }
    fn parse_ssh_user(vars: &Vars) -> String {
        if let Some(Value::String(v)) = vars.get("ssh.user") {
            return v.clone();
        }
        "root".to_string()
    }
    pub fn run(&self, inventory: &str, host: &str, vars: &Vars) -> Result<()> {
        debug!("host {} env: {:?}", host, vars);
        let user = Self::parse_ssh_user(vars);
        let port = Self::parse_ssh_port(vars);
        let key: String = Self::parse_ssh_key_file(inventory, vars);

        let ssh = format!(
            "ssh -o StrictHostKeyChecking=no -p {port} -i {key}",
            port = port,
            key = key
        );
        match self {
            Self::Upload { src, dest } => {
                let src = template_file(inventory, src, vars)?.display().to_string();
                let dest = template_str(dest, vars)?;
                if host == Self::LOCALHOST {
                    shell(
                        host,
                        ShellCommand::new("rsync")
                            .arg("-rlptD")
                            .arg("-v")
                            .arg(src)
                            .arg(dest),
                    )?;
                } else {
                    shell(
                        host,
                        ShellCommand::new("rsync")
                            .arg("-rlptD")
                            .arg("-zz")
                            .arg("-v")
                            .arg("-e")
                            .arg(ssh)
                            .arg(src)
                            .arg(format!(
                                "{user}@{host}:{dest}",
                                user = user,
                                host = host,
                                dest = dest,
                            )),
                    )?;
                }
            }
            Self::Download { src, dest } => {
                let src = template_str(src, vars)?;
                let dest = template_str(dest, vars)?;
                let dest = {
                    let it = Path::new("tmp").join("downloads").join(host).join(dest);
                    {
                        if let Some(it) = it.parent() {
                            if !it.exists() {
                                create_dir_all(it)?;
                            }
                        }
                    }
                    it.display().to_string()
                };
                if host == Self::LOCALHOST {
                    shell(
                        host,
                        ShellCommand::new("rsync")
                            .arg("-rlptD")
                            .arg("-v")
                            .arg(src)
                            .arg(dest),
                    )?;
                } else {
                    shell(
                        host,
                        ShellCommand::new("rsync")
                            .arg("-rlptD")
                            .arg("-zz")
                            .arg("-v")
                            .arg("-e")
                            .arg(ssh)
                            .arg(format!(
                                "{user}@{host}:{src}",
                                src = src,
                                user = user,
                                host = host,
                            ))
                            .arg(dest),
                    )?;
                }
            }
            Self::Shell { script } => {
                let script = template_file(inventory, script, vars)?
                    .display()
                    .to_string();
                if host == Self::LOCALHOST {
                    shell(host, ShellCommand::new("bash").arg(script))?;
                } else {
                    shell(
                        host,
                        ShellCommand::new("ssh")
                            .arg("-o")
                            .arg("StrictHostKeyChecking=no")
                            .arg("-p")
                            .arg(port.to_string())
                            .arg("-i")
                            .arg(key)
                            .arg(format!("{}@{}", user, host))
                            .arg("bash -s")
                            .stdin(Stdio::from(File::open(script)?)),
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

fn shell(host: &str, cmd: &mut ShellCommand) -> Result<()> {
    let root = Path::new("tmp").join("logs");
    if !root.exists() {
        create_dir_all(&root)?;
    }
    let outputs = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(root.join(host))?;
    {
        let mut wrt = BufWriter::new(&outputs);
        writeln!(wrt, "{}: {:?}", Utc::now().naive_local(), cmd)?;
    }
    let errors = outputs.try_clone()?;

    let out = cmd
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;
    if !out.status.success() {
        return Err(format_err!("{:?}", cmd));
    }
    Ok(())
}

fn template_file<P: AsRef<Path>>(inventory: &str, tpl: P, vars: &Vars) -> Result<PathBuf> {
    let tpl = tpl.as_ref();
    {
        let tpl = Path::new(inventory).join(tpl);
        if tpl.exists() {
            return Ok(tpl.to_path_buf());
        }
    }
    {
        let tpl = Path::new(JOBS).join(tpl);
        if tpl.exists() {
            return Ok(tpl.to_path_buf());
        }
        {
            let tpl = tpl.with_extension(TEMPLATE_EXT);
            if tpl.exists() {
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
                return Ok(rdr);
            }
        }
    }

    let buf = template_str(&tpl.display().to_string(), vars)?;
    Ok(Path::new(&buf).to_path_buf())
}

fn template_str(tpl: &str, vars: &Vars) -> Result<String> {
    let mut reg = Handlebars::new();
    let name = "";
    reg.set_strict_mode(true);
    reg.register_template_string(name, tpl)?;
    Ok(reg.render(name, vars)?)
}
