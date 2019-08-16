use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, File};
use std::io::prelude::*;
use std::path::Path;
use std::result::Result as StdResult;

use serde::{de::DeserializeOwned, ser::Serialize};
use tera::{Context, Tera};
use uuid::Uuid;

use super::{
    agent::{Payload, Task as AgentTask},
    errors::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub agents: Vec<String>,
    pub environment: HashMap<String, String>,
}

impl Group {
    pub fn new<P: AsRef<Path>, T: AsRef<str>>(etc: P, name: T) -> Result<Self> {
        let mut file = etc.as_ref().join("groups").join(name.as_ref());
        file.set_extension("toml");
        info!("load group from {}", file.display());
        parse(file)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Task {
    Script {
        user: String,
        file: String,
    },
    Upload {
        source: String,
        target: String,
        owner: Option<String>,
        group: Option<String>,
        mode: Option<u32>,
    },
}

impl Task {
    fn folder<P: AsRef<Path>, S: Serialize>(
        items: &mut Vec<Payload>,
        source: &String,
        file: P,
        env: &S,
        target: &String,
    ) -> Result<()> {
        let file = file.as_ref();
        info!("find folder {}", file.display());
        for entry in read_dir(file)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                Self::folder(items, source, &path, env, target)?;
            } else {
                let name = path.strip_prefix(source)?.to_str().unwrap().to_string();
                items.push(Self::file(path, env, &(target.clone() + "/" + &name))?);
            }
        }
        Ok(())
    }
    fn file<P: AsRef<Path>, S: Serialize>(file: P, env: &S, target: &String) -> Result<Payload> {
        let body = {
            let mut file = file.as_ref().to_path_buf();
            if file.exists() {
                info!("read file {}", file.display());
                let mut file = File::open(file)?;
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                buf
            } else {
                file.set_extension(TEMPLATE_EXT);
                info!("parse file {}", file.display());
                let buf = template(file, env)?;
                buf.into_bytes()
            }
        };
        Ok(Payload::File {
            path: target.to_string(),
            body: body,
        })
    }
    fn files<P: AsRef<Path>, S: Serialize>(
        root: P,
        env: &S,
        source: &String,
        target: &String,
        owner: &Option<String>,
        group: &Option<String>,
        mode: &Option<u32>,
    ) -> Result<Vec<Payload>> {
        let mut items = Vec::new();

        let file = root.as_ref().join(source);
        if file.is_dir() {
            if file.is_relative() {
                return Err(format_err!("folder path must be absolute"));
            }
            let mut children = Vec::new();
            Self::folder(&mut children, source, file, env, target)?;
            items.extend(children);
        } else {
            items.push(Self::file(file, env, target)?);
        }

        if let Some(owner) = owner {
            items.push(Payload::Shell {
                user: ROOT.to_string(),
                script: format!("chown {} {}", owner, target),
            });
        }
        if let Some(group) = group {
            items.push(Payload::Shell {
                user: ROOT.to_string(),
                script: format!("chgrp {} {}", group, target),
            });
        }
        if let Some(mode) = mode {
            items.push(Payload::Shell {
                user: ROOT.to_string(),
                script: format!("chmod {:o} {}", mode, target),
            });
        }
        Ok(items)
    }
    fn script<P: AsRef<Path>, S: Serialize>(
        etc: P,
        env: &S,
        user: &String,
        file: &String,
    ) -> Result<Payload> {
        let mut file = etc.as_ref().join(file);
        let script = if file.exists() {
            info!("read file {}", file.display());
            read_to_string(file)?
        } else {
            file.set_extension(TEMPLATE_EXT);
            info!("parse file {}", file.display());
            template(file, env)?
        };
        Ok(Payload::Shell {
            user: user.to_string(),
            script: script,
        })
    }
}

impl AgentTask {
    pub fn new<P: AsRef<Path>, T: AsRef<str>, S: Serialize>(
        var: P,
        name: T,
        env: &S,
    ) -> Result<Self> {
        let etc = var.as_ref().join("tasks").join(name.as_ref());
        info!("load task from {}", etc.display());
        info!("parse readme");
        let readme = template(etc.join("readme.json"), env)?;
        let items: Vec<Task> = serde_json::from_str(&readme)?;

        let mut payload = Vec::new();
        for it in items {
            payload.append(&mut Payload::new(&etc, &it, env)?);
        }

        Ok(Self {
            id: Uuid::new_v4(),
            payload: payload,
        })
    }
}

const TEMPLATE_EXT: &'static str = "tera";
const ROOT: &'static str = "root";

impl Payload {
    pub fn new<P: AsRef<Path>, S: Serialize>(etc: P, task: &Task, env: &S) -> Result<Vec<Self>> {
        let mut items = Vec::new();
        match task {
            Task::Script { user, file } => {
                items.push(Task::script(etc, env, &user, &file)?);
            }
            Task::Upload {
                source,
                target,
                owner,
                group,
                mode,
            } => {
                items.extend(Task::files(
                    etc, env, &source, &target, &owner, &group, &mode,
                )?);
            }
        };
        Ok(items)
    }
}

fn template<P: AsRef<Path>, V: Serialize>(file: P, args: &V) -> StdResult<String, Error> {
    let name = "this";
    let mut tpl = Tera::default();
    tpl.add_template_file(file.as_ref(), Some(name))?;
    let buf = tpl.render(name, Context::from_serialize(args)?)?;
    Ok(buf)
}

fn parse<P: AsRef<Path>, T: DeserializeOwned>(file: P) -> Result<T> {
    let mut file = File::open(file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let it = toml::from_slice(&buf)?;
    Ok(it)
}
