use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use clap::App;
use rocket::config::{Config, Environment, Value};
use rocket_contrib::serve::StaticFiles;
use sodiumoxide::randombytes;

use super::{
    controllers, crawler,
    env::{etc, third, var, NAME, VERSION},
    errors::Result,
    orm::{open as open_db, Database},
};

pub fn launch() -> Result<()> {
    let etc = etc();
    let var = var();

    log4rs::init_file(etc.join("log4rs.yml"), Default::default())?;

    App::new(NAME)
        .version(VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .before_help(include_str!("banner.txt"))
        .after_help(env!("CARGO_PKG_HOMEPAGE"))
        .get_matches();

    let mut databases = HashMap::new();
    {
        let db = var.join("db").display().to_string();
        {
            let db = open_db(&db)?;
            info!("start clawer thread");
            thread::spawn(move || loop {
                if let Err(e) = crawler::run(&db) {
                    error!("{:?}", e);
                }
                thread::sleep(Duration::from_secs(60 * 5));
            });
        }
        let mut it = HashMap::new();
        it.insert("url", Value::from(db));
        databases.insert("sqlite", Value::from(it));
    }

    rocket::custom(
        Config::build(Environment::Production)
            .address("127.0.0.1")
            .port(8080)
            .secret_key(base64::encode(&randombytes::randombytes(32)))
            .workers(1 << 5)
            .extra("databases", databases)
            .finalize()?,
    )
    .attach(Database::fairing())
    .mount("/", routes![controllers::index])
    .mount("/3rd", StaticFiles::from(third()))
    .launch();
    Ok(())
}
