use std::collections::BTreeMap;

use super::errors::Result;

pub const CONFIG_EXT: &str = "toml";
pub const TEMPLATE_EXT: &str = "hbs";

pub type Vars = BTreeMap<String, String>;
pub type Excutor = (BTreeMap<String, Vars>, Vec<Task>);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub groups: Vec<String>,
    pub vars: Vars,
    pub tasks: Vec<Task>,
}

impl Job {
    pub fn load(_name: &str) -> Result<Vec<Excutor>> {
        let items = Vec::new();
        Ok(items)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub name: String,
    pub groups: Vec<String>,
    pub commands: Vec<Command>,
}

impl Task {
    pub fn run(&self, _host: &str, _vars: &Vars) -> Result<()> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Command {
    Upload { src: String, dest: String },
    Download { src: String, dest: String },
    Shell(String),
}
