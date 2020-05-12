use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

use clap::{App, Arg};
use failure::Error;

use super::{
    env,
    errors::Result,
    models::{Host, Role},
    orm,
};

pub fn run() -> Result<()> {
    let matches = App::new(env::NAME)
        .version(&*format!("{}({})", env::VERSION, env::BUILD_TIME))
        .author(env::AUTHORS)
        .about(env::DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(env::HOMEPAGE)
        .arg(
            Arg::with_name("inventory")
                .short("i")
                .long("inventory")
                .value_name("INVENTORY")
                .help("Inventory name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("role")
                .short("r")
                .long("role")
                .value_name("ROLE")
                .help("Role name")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let inventory = matches.value_of("inventory").unwrap();
    let role = matches.value_of("role").unwrap();

    let db = orm::open()?;
    let reason = Arc::new(Mutex::new(None::<Error>));

    let jobs = Role::load(inventory, role)?;
    for (job, hosts, tasks) in jobs {
        {
            let reason = reason.lock();
            if let Ok(ref reason) = reason {
                if let Some(ref e) = reason.deref() {
                    return Err(format_err!("{}", e));
                }
            }
        }
        info!("get job {}", job);
        let mut children = vec![];

        for (host, vars) in hosts {
            let job = job.clone();
            let host = host.clone();
            let vars = vars.clone();
            let tasks = tasks.clone();
            let reason = reason.clone();
            children.push(
                thread::Builder::new()
                    .name(format!("{} - {}", host, job))
                    .spawn(move || {
                        let reason = reason.clone();
                        if let Err(e) = Host::handle(&host, &vars, &tasks) {
                            if let Ok(mut reason) = reason.lock() {
                                *reason = Some(e);
                            }
                        }
                    })?,
            );
        }
        for it in children {
            let _ = it.join();
        }
    }

    info!("Done.");
    Ok(())
}
