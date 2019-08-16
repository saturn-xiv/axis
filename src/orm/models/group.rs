use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::errors::Result;
use super::super::{schema::agents, Connection};

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_name(&self, agent: i32) -> Result<Item>;
    fn by_agent(&self, agent: i32) -> Result<Vec<Item>>;
    fn all(&self) -> Result<Vec<Item>>;
    fn create(&self, name: &String) -> Result<()>;
    fn update(&self, name: &String) -> Result<()>;
    fn delete(&self, id: i32) -> Result<()>;
    fn bind(&self, group: i32, agent: i32) -> Result<()>;
    fn unbind(&self, group: i32, agent: i32) -> Result<()>;
}
