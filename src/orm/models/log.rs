use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*};

use super::super::super::errors::Result;
use super::super::{schema::logs, Connection};

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: i32,
    pub map: String,
    pub ip: String,
    pub task: String,
    pub message: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_mac(&self, mac: &String) -> Result<Vec<Item>>;
    fn all(&self) -> Result<Vec<Item>>;
    fn create(&self, mac: &String, ip: &String, task: &String, message: &String) -> Result<()>;
}

impl Dao for Connection {
    fn by_mac(&self, mac: &String) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::mac.eq(mac))
            .order(logs::dsl::created_at.desc())
            .load(self)?;
        Ok(items)
    }
    fn all(&self) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .order(logs::dsl::created_at.desc())
            .load(self)?;
        Ok(items)
    }
    fn create(&self, mac: &String, ip: &String, task: &String, message: &String) -> Result<()> {
        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::mac.eq(mac),
                logs::dsl::ip.eq(ip),
                logs::dsl::task.eq(task),
                logs::dsl::message.eq(message),
                logs::dsl::created_at.eq(&Utc::now().naive_utc()),
            ))
            .execute(self)?;
        Ok(())
    }
}
