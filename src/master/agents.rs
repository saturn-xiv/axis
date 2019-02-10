use super::super::{
    errors::Result,
    orm::{models::Dao, Connection},
};

pub fn list(db: Connection) -> Result<()> {
    let items = db.list()?;
    println!("ENABLE\tNAME");
    for it in items {
        println!("{}\t{}", if it.enable { "YES" } else { "NO" }, it.sn);
    }
    Ok(())
}

pub fn accept(db: Connection, sn: &str) -> Result<()> {
    let it = db.by_sn(sn)?;
    info!("accept agent {}", it.sn);
    db.enable(it.id, true)?;
    Ok(())
}

pub fn reject(db: Connection, sn: &str) -> Result<()> {
    let it = db.by_sn(sn)?;
    info!("reject agent {}", it.sn);
    db.enable(it.id, false)?;
    Ok(())
}

pub fn delete(db: Connection, sn: &str) -> Result<()> {
    let it = db.by_sn(sn)?;
    info!("delete agent {}", it.sn);
    db.delete(it.id)?;
    Ok(())
}
