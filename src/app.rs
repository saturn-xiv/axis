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
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Http listening port")
                .takes_value(true),
        )
        .get_matches();

    let port: u16 = matches.value_of("port").unwrap_or("8080").parse()?;

    let db = orm::open()?;
    Ok(())
}

fn run_task(inventory: &str, role: &str) -> Result<()> {
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
