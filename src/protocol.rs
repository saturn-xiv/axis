use uuid::Uuid;

use super::key::KEY;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Request {
    Register((String, KEY)),
    Report((String, Uuid, String)),
}
