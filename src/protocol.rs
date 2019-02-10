use uuid::Uuid;

use super::key::Key;

#[derive(Serialize, Deserialize)]
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
