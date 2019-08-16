use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::errors::Result;
use super::super::{schema::agents, Connection};

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: i32,
    pub agent_id: String,
    pub ip: String,
    pub task: String,
    pub message: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_agent(&self, agent: i32) -> Result<Vec<Item>>;
    fn all(&self) -> Result<Vec<Item>>;
    fn create(&self, agent: i32, ip: &String, task: &String, message: &String) -> Result<()>;
}
