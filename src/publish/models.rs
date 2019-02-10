use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

use serde::ser::Serialize;
use uuid::Uuid;

use super::super::{
    agent::task::{Payload, Task as AgentTask},
    errors::{Error, Result},
    parse, NAME,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub agents: Vec<String>,
    pub environment: HashMap<String, String>,
}

impl Group {
    pub fn new<T: AsRef<str>>(name: T) -> Result<Self> {
        let mut file = root().join("groups").join(name.as_ref());
        file.set_extension("toml");
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

impl AgentTask {
    pub fn new<T: AsRef<str>, S: Serialize>(name: T, env: &S) -> Result<Self> {
        let etc = root().join("tasks").join(name.as_ref());
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
                let mut file = etc.as_ref().join(file);
                if file.exists() {
                    info!("read file {}", file.display());
                    items.push(Payload::Shell {
                        user: user.to_string(),
                        script: read_to_string(file)?,
                    });
                } else {
                    file.set_extension(TEMPLATE_EXT);
                    info!("parse file {}", file.display());
                    let script = template(file, env)?;
                    items.push(Payload::Shell {
                        user: user.to_string(),
                        script: script,
                    });
                }
            }
            Task::Upload {
                source,
                target,
                owner,
                group,
                mode,
            } => {
                let body = {
                    let mut file = etc.as_ref().join(source);
                    if file.exists() {
                        info!("read file {}", file.display());
                        let mut file = File::open(file)?;
                        let mut buf = Vec::new();
                        file.read_to_end(&mut buf)?;
                        buf
                    } else {
                        file.set_extension(TEMPLATE_EXT);
                        info!("parse file {}", file.display());
                        let script = template(file, env)?;
                        script.into_bytes()
                    }
                };
                items.push(Payload::File {
                    path: target.to_string(),
                    body: body,
                });
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
            }
        };
        Ok(items)
    }
}

fn template<P: AsRef<Path>, V: Serialize>(file: P, args: &V) -> StdResult<String, Error> {
    let name = "this";
    let mut tpl = tera::Tera::default();
    tpl.add_template_file(file.as_ref(), Some(name))?;
    let buf = tpl.render_value(name, args)?;
    Ok(buf)
}

fn root() -> PathBuf {
    let mut d = Path::new("/etc").join("lib").join(NAME);
    if !d.exists() {
        d = Path::new(".etc").to_path_buf();
    }
    d
}
