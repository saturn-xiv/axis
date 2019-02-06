use super::env::Group;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub tasks: Vec<Group>,
}
