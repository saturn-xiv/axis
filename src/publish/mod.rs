pub mod models;

use std::path::PathBuf;

use super::{
    agent::task::Task as AgentTask,
    errors::Result,
    key::Pair,
    master::{Config, CONFIG_FILE, KEY_FILE},
    orm::{models::Dao, Connection},
};

use zmq::{Context, PUB, SNDMORE};

pub fn launch(etc: PathBuf, group: &str, task: &str, db: Connection) -> Result<()> {
    let key = Pair::new(&etc.join(KEY_FILE))?;
    let cfg: Config = super::parse(etc.join(CONFIG_FILE))?;
    let group = models::Group::new(group)?;

    let task = AgentTask::new(task, &group.environment)?;
    let task = rmp_serde::encode::to_vec(&task)?;

    let ctx = Context::default();
    let publisher = ctx.socket(PUB)?;

    publisher.connect(&format!("tcp://*:{}", cfg.port.publisher()))?;

    for it in group.agents {
        let it = db.by_sn(it)?;
        if it.enable {
            publisher.set_curve_serverkey(&it.finger()?.0)?;
            publisher.set_curve_publickey(&key.public.0)?;
            publisher.set_curve_secretkey(&key.private.0)?;
            publisher.send(&it.sn, SNDMORE)?;
            publisher.send(&task, 0)?;
        } else {
            warn!("agent {} isn't enable", it.sn);
        }
    }
    Ok(())
}
