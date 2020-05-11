use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

use clap::{App, Arg};

use super::{
    env,
    errors::Result,
    models::{Host, Report, Role},
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

    let status: BTreeMap<String, Report> = BTreeMap::new();
    let status = Arc::new(Mutex::new(status));

    let jobs = Role::load(inventory, role)?;
    for (job, hosts, tasks) in jobs {
        let mut children = vec![];

        for (host, vars) in hosts {
            let status = status.clone();
            if let Ok(status) = status.lock() {
                if let Some(it) = status.get(&host) {
                    if let Some(e) = &it.reason {
                        return Err(format_err!("host {} {}", host, e));
                    }
                }
            }
            let job = job.clone();
            let host = host.clone();
            let vars = vars.clone();
            let tasks = tasks.clone();
            children.push(thread::spawn(move || {
                let reason = match Host::handle(&host, &vars, &tasks) {
                    Ok(()) => None,
                    Err(e) => {
                        error!("{} {:?}", host, e);
                        Some(e)
                    }
                };
                if let Ok(ref mut status) = status.lock() {
                    status.insert(host.clone(), Report::new(job.clone(), reason));
                }
            }));
        }
        for it in children {
            let _ = it.join();
        }
    }

    println!("{:16} REPORT", "HOST");
    if let Ok(status) = status.lock() {
        for (h, r) in status.iter() {
            println!("{:16} {}", h, r);
        }
    }
    Ok(())
}
