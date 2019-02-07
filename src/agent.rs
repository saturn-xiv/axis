use std::default::Default;
use std::fmt;
use std::path::PathBuf;

use zmq::{Context, REQ, SUB};

use super::{
    errors::Result,
    key::{Pair, KEY},
    protocol::Request,
    task::Task,
    Port,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub id: String,
    pub master: Master,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Master {
    pub port: Port,
    pub host: String,
    pub finger: String,
}

impl Master {
    pub fn finger(&self) -> Result<KEY> {
        let buf: Vec<u8> = base64::decode(&self.finger)?;

        let mut key: KEY = Default::default();
        let len = key.len();
        key.copy_from_slice(&buf[0..len]);
        Ok(key)
    }
}

impl fmt::Display for Master {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port.publisher())
    }
}

const KEY_FILE: &'static str = "agent.key";

pub fn launch(etc: PathBuf) -> Result<()> {
    let key = Pair::new(&etc.join(KEY_FILE))?;
    let cfg: Config = super::parse(etc.join("agent.toml"))?;
    info!("register to master {} with id {}", cfg.master, cfg.id);

    let ctx = Context::new();

    let req = ctx.socket(REQ)?;
    req.set_curve_serverkey(&cfg.master.finger()?)?;
    req.set_curve_publickey(&key.public.0)?;
    req.set_curve_secretkey(&key.private.0)?;
    req.send(
        &rmp_serde::encode::to_vec(&Request::Register((cfg.id.clone(), cfg.master.finger()?)))?,
        0,
    )?;

    let sub = ctx.socket(SUB)?;
    sub.set_curve_server(true)?;
    sub.set_curve_secretkey(&cfg.master.finger()?)?;
    sub.connect(&format!(
        "{}:{}",
        cfg.master.host,
        cfg.master.port.publisher()
    ))?;
    sub.set_subscribe(cfg.id.as_bytes())?;

    loop {
        let env = String::from_utf8(sub.recv_bytes(0)?)?;
        let task: Task = rmp_serde::decode::from_slice(&sub.recv_bytes(0)?)?;
        info!("receive from {} \n{}", env, task);
        let res = task.payload.execute();
        info!("{:?}", res);
        req.send(
            &rmp_serde::encode::to_vec(&Request::Report((
                cfg.id.clone(),
                task.id,
                format!("{:?}", res),
            )))?,
            0,
        )?;
    }
}

pub fn finger(etc: PathBuf) -> Result<()> {
    let key = Pair::new(&etc.join(KEY_FILE))?;
    println!("{}", key);
    Ok(())
}
