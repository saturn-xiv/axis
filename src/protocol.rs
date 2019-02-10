use std::fmt;
use uuid::Uuid;

use super::key::Key;

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
        }
    }
}
