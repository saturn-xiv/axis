use std::thread;

use clap::{App, Arg};

use super::{
    env,
    errors::Result,
    models::{Host, Job},
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
            Arg::with_name("job")
                .short("j")
                .long("job")
                .value_name("JOB")
                .help("Job name")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let inventory = matches.value_of("inventory").unwrap();
    let job = matches.value_of("job").unwrap();

    let jobs = Job::load(inventory, job)?;
    for (hosts, tasks) in jobs {
        let mut children = vec![];

        for (host, vars) in hosts {
            let host = host.clone();
            let vars = vars.clone();
            let tasks = tasks.clone();
            children.push(thread::spawn(move || {
                if let Err(e) = Host::handle(&host, &vars, &tasks) {
                    error!("{} {:?}", host, e);
                }
            }));
        }
        for it in children {
            let _ = it.join();
        }
    }
    Ok(())
}
