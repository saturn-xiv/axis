use std::fmt;
use uuid::Uuid;

use super::{agent::task::Task as AgentTask, key::Key};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Request {
    Register {
        host: String,
        finger: Key,
    },
    Report {
        host: String,
        task: Uuid,
        result: String,
    },
    Publish {
        secret: String,
        agents: Vec<String>,
        task: AgentTask,
    },
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Request::Register {
                ref host,
                ref finger,
            } => write!(f, "register {} {}", host, finger),
            Request::Report {
                ref host,
                ref task,
                ref result,
            } => write!(f, "report {}@{}\n{}", host, task, result),
            Request::Publish {
                secret: _,
                ref agents,
                ref task,
            } => write!(f, "publish {:?}\n{}", agents, task),
        }
    }
}
