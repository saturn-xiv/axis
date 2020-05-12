use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use uuid::Uuid;

use super::super::super::errors::Result;
use super::super::{schema::logs, Connection};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub uid: String,
    pub host: String,
    pub job: String,
    pub task: String,
    pub result: Option<String>,
    pub updated: NaiveDateTime,
    pub created: NaiveDateTime,
}

impl Item {
    pub fn uid() -> String {
        Utc::now().format("%y%m%d%H%M%S%3f").to_string()
    }
}

pub trait Dao {
    fn all(&self) -> Result<Vec<String>>;
    fn by_uid(&self, uid: &str) -> Result<Vec<Item>>;
    fn by_host(&self, host: &str) -> Result<Vec<Item>>;
    fn add(&self, uid: &str, host: &str, job: &str, task: &str) -> Result<String>;
    fn set(&self, id: &str, result: &str) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self) -> Result<Vec<String>> {
        let items = logs::dsl::logs
            .select(logs::dsl::uid)
            .distinct()
            .load::<String>(self)?;
        Ok(items)
    }
    fn by_uid(&self, uid: &str) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::uid.eq(uid))
            .order(logs::dsl::updated.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_host(&self, host: &str) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::host.eq(host))
            .order(logs::dsl::updated.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn add(&self, uid: &str, host: &str, job: &str, task: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().naive_utc();
        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::id.eq(&id),
                logs::dsl::uid.eq(uid),
                logs::dsl::host.eq(host),
                logs::dsl::job.eq(job),
                logs::dsl::task.eq(task),
                logs::dsl::updated.eq(&now),
            ))
            .execute(self)?;
        Ok(id)
    }
    fn set(&self, id: &str, result: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = logs::dsl::logs.filter(logs::dsl::id.eq(id));
        update(it)
            .set((
                logs::dsl::result.eq(Some(result.to_string())),
                logs::dsl::updated.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
