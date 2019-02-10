pub mod task;

use std::default::Default;
use std::fmt;
use std::path::PathBuf;

use zmq::{Context, Socket, REQ, SUB};

use super::{
    errors::Result,
    key::{Pair, KEY},
    protocol::Request,
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

pub fn launch(etc: PathBuf) -> Result<()> {
    let key = Pair::new(&etc.join("agent.key"))?;
    let cfg: Config = super::parse(etc.join("agent.toml"))?;
    info!("register to master {} with id {}", cfg.master, cfg.id);
    let req = reporter_socket(&cfg, &key)?;
    send_report(
        &req,
        &Request::Register {
            host: cfg.id.clone(),
            finger: key.public.clone(),
        },
    )?;

    let ctx = Context::new();
    let sub = ctx.socket(SUB)?;

    sub.set_curve_serverkey(&cfg.master.finger()?)?;
    sub.set_curve_publickey(&key.public.0)?;
    sub.set_curve_secretkey(&key.private.0)?;
    let url = format!("tcp://{}:{}", cfg.master.host, cfg.master.port.publisher());
    info!("connect to publisher {}", url);
    sub.connect(&url)?;
    sub.set_subscribe(cfg.id.as_bytes())?;

    loop {
        let env = String::from_utf8(sub.recv_bytes(0)?)?;
        let task: task::Task = rmp_serde::decode::from_slice(&sub.recv_bytes(0)?)?;
        info!("receive from {} \n{}", env, task);
        let res: Vec<Result<String>> = task.payload.iter().map(|it| it.execute()).collect();
        info!("{:?}", res);
        send_report(
            &req,
            &Request::Report {
                host: cfg.id.clone(),
                task: task.id,
                result: format!("{:?}", res),
            },
        )?;
    }
}

fn reporter_socket(cfg: &Config, key: &Pair) -> Result<Socket> {
    let ctx = Context::new();
    let req = ctx.socket(REQ)?;
    req.set_curve_serverkey(&cfg.master.finger()?)?;
    req.set_curve_publickey(&key.public.0)?;
    req.set_curve_secretkey(&key.private.0)?;
    let url = format!("tcp://{}:{}", cfg.master.host, cfg.master.port.reporter());
    info!("connect to reporter {}", url);
    req.connect(&url)?;
    Ok(req)
}

fn send_report(s: &Socket, r: &Request) -> Result<()> {
    info!("send {}", r);
    s.send(&rmp_serde::encode::to_vec(r)?, 0)?;
    let buf = s.recv_bytes(0)?;
    info!("receive {}", String::from_utf8(buf)?);
    Ok(())
}
