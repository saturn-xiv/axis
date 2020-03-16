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
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn all(&self) -> Result<Vec<Item>>;
    fn create(&self, mac: &String, ip: &String, name: &String) -> Result<()>;
    fn update(&self, id: i32, ip: &String, name: &String) -> Result<()>;
    fn by_mac(&self, mac: &String) -> Result<Item>;
    fn clear(&self) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self) -> Result<Vec<Item>> {
        let items = agents::dsl::agents
            .order(agents::dsl::name.asc())
            .load(self)?;
        Ok(items)
    }
    fn create(&self, mac: &String, ip: &String, name: &String) -> Result<()> {
        insert_into(agents::dsl::agents)
            .values((
                agents::dsl::mac.eq(mac),
                agents::dsl::ip.eq(ip),
                agents::dsl::name.eq(name),
                agents::dsl::updated_at.eq(&Utc::now().naive_utc()),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&self, id: i32, ip: &String, name: &String) -> Result<()> {
        update(agents::dsl::agents.filter(agents::dsl::id.eq(id)))
            .set((
                agents::dsl::ip.eq(ip),
                agents::dsl::name.eq(name),
                agents::dsl::updated_at.eq(&Utc::now().naive_utc()),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_mac(&self, mac: &String) -> Result<Item> {
        let it = agents::dsl::agents
            .filter(agents::dsl::mac.eq(mac))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn clear(&self) -> Result<()> {
        delete(agents::dsl::agents).execute(self)?;
        Ok(())
    }
}
