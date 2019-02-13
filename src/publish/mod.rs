pub mod models;

use std::path::PathBuf;

use super::{
    agent::{send_report, task::Task as AgentTask},
    errors::Result,
    key::Pair,
    master::{Config, CONFIG_FILE, KEY_FILE},
    orm::Connection,
    protocol::Request,
};

use zmq::{Context, REQ};

pub fn launch(etc: PathBuf, var: PathBuf, group: &str, task: &str, _db: Connection) -> Result<()> {
    let key = Pair::new(&etc.join(KEY_FILE))?;
    let cfg: Config = super::parse(etc.join(CONFIG_FILE))?;
    let group = models::Group::new(&var, group)?;

    let task = AgentTask::new(&var, task, &group.environment)?;
    info!("{}", task);

    let ctx = Context::default();
    let req = ctx.socket(REQ)?;
    req.set_curve_serverkey(&key.public.0)?;
    req.set_curve_publickey(&key.public.0)?;
    req.set_curve_secretkey(&key.private.0)?;

    let url = format!("tcp://localhost:{}", cfg.port.reporter());
    info!("connect to {}", url);
    req.connect(&url)?;

    send_report(
        &req,
        &Request::Publish {
            secret: cfg.secret.clone(),
            agents: group.agents,
            task: task,
        },
    )?;

    info!("Done.");
    Ok(())
}
