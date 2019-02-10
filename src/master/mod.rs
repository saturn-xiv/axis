pub mod agents;

use std::path::PathBuf;

use zmq::{Context, PUB, REP};

use super::{
    errors::Result,
    key::{Key, Pair},
    orm::{models::Dao, Connection},
    protocol::Request,
    Port,
};

pub const KEY_FILE: &'static str = "master.key";
pub const CONFIG_FILE: &'static str = "master.toml";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub port: Port,
}

pub fn launch(etc: PathBuf, db: Connection) -> Result<()> {
    let key = Pair::new(&etc.join(KEY_FILE))?;
    let cfg: Config = super::parse(etc.join(CONFIG_FILE))?;

    let ctx = Context::default();
    let publisher = ctx.socket(PUB)?;
    info!(
        "start publisher on http://localhost:{}",
        cfg.port.publisher()
    );
    publisher.bind(&format!("tcp://*:{}", cfg.port.publisher()))?;
    info!("start reporter on http://localhost:{}", cfg.port.reporter());
    loop {
        if let Err(e) = reporter(&cfg, &key, &db) {
            error!("###{:?}", e);
        }
    }
}

fn reporter(cfg: &Config, key: &Pair, db: &Connection) -> Result<()> {
    let ctx = Context::default();
    let rep = ctx.socket(REP)?;
    rep.set_curve_server(true)?;
    rep.set_curve_secretkey(&key.private.0)?;
    rep.bind(&format!("tcp://*:{}", cfg.port.reporter()))?;

    loop {
        let buf = rep.recv_bytes(0)?;
        let req: Request = rmp_serde::decode::from_slice(&buf)?;

        let finger = match req {
            Request::Register { host, finger } => {
                info!("register {}", host);
                match db.by_sn(&host) {
                    Ok(_) => info!("agent {} already exist", host),
                    Err(_) => {
                        info!("can't find agent {}, will add it", host);
                        db.add(&host, &finger.to_string())?;
                    }
                };
                finger
            }
            Request::Report { host, task, result } => {
                info!("{}@{}\n{}", task, host, result);
                db.by_sn(&host)?.finger.parse::<Key>()?
            }
        };
        rep.set_curve_serverkey(&finger.0)?;
        rep.set_curve_publickey(&key.public.0)?;
        rep.set_curve_secretkey(&key.private.0)?;
        rep.send("Ok", 0)?;
    }
}
