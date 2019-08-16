use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::errors::Result;
use super::super::{schema::agents, Connection};

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: i32,
    pub mac: String,
    pub ip: String,
    pub name: String,
    pub hardware: String,
    pub os: String,
    pub version: Option<String>,
    pub online: bool,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_group(&self, group: i32) -> Result<Vec<Item>>;
    fn all(&self) -> Result<Vec<Item>>;
    fn create(
        &self,
        mac: &String,
        ip: &String,
        name: &String,
        hardware: &String,
        os: &String,
        version: &Option<String>,
    ) -> Result<()>;
    fn update(
        &self,
        id: i32,
        ip: &String,
        name: &String,
        hardware: &String,
        os: &String,
        version: &Option<String>,
    ) -> Result<()>;
    fn by_mac(&self, mac: &String) -> Result<Item>;
    fn clear(&self) -> Result<()>;
}
