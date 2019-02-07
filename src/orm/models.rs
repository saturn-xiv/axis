use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::errors::Result;
use super::{schema::agents, Connection};

#[derive(Queryable, Serialize)]
pub struct Agent {
    pub id: i32,
    pub sn: String,
    pub key: String,
    pub enable: bool,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn list(&self) -> Result<Vec<Agent>>;
    fn add(&self, sn: &String, key: &String) -> Result<()>;
    fn by_sn<S: AsRef<str>>(&self, sn: S) -> Result<Agent>;
    fn delete(&self, id: i32) -> Result<()>;
    fn enable(&self, id: i32, ok: bool) -> Result<()>;
}

impl Dao for Connection {
    fn list(&self) -> Result<Vec<Agent>> {
        let items = agents::dsl::agents
            .order(agents::dsl::sn.asc())
            .load::<Agent>(self)?;
        Ok(items)
    }
    fn add(&self, sn: &String, key: &String) -> Result<()> {
        insert_into(agents::dsl::agents)
            .values((
                agents::dsl::sn.eq(sn),
                agents::dsl::key.eq(key),
                agents::dsl::updated_at.eq(&Utc::now().naive_utc()),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_sn<S: AsRef<str>>(&self, sn: S) -> Result<Agent> {
        let it = agents::dsl::agents
            .filter(agents::dsl::sn.eq(sn.as_ref()))
            .first::<Agent>(self)?;
        Ok(it)
    }
    fn delete(&self, id: i32) -> Result<()> {
        delete(agents::dsl::agents.filter(agents::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
    fn enable(&self, id: i32, ok: bool) -> Result<()> {
        update(agents::dsl::agents.filter(agents::dsl::id.eq(id)))
            .set((
                agents::dsl::enable.eq(ok),
                agents::dsl::updated_at.eq(&Utc::now().naive_utc()),
            ))
            .execute(self)?;
        Ok(())
    }
}
