use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

use clap::{App, Arg};

use super::{
    env,
    errors::{Error, Result},
    models::Job,
};

pub fn run() -> Result<()> {
    let matches = App::new(env::NAME)
        .version(&*format!("{}({})", env::VERSION, env::BUILD_TIME))
        .author(env::AUTHORS)
        .about(env::DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(env::HOMEPAGE)
        .arg(
            Arg::with_name("job")
                .short("j")
                .long("job")
                .value_name("JOB")
                .help("Job")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("inventory")
                .short("i")
                .long("inventory")
                .value_name("INVENTORY")
                .help("Inventory")
                .takes_value(true),
        )
        .get_matches();

    let job = matches
        .value_of("job")
        .ok_or_else(|| Error::Custom("please give a job name".to_string()))?;

    let inventory = matches
        .value_of("inventory")
        .ok_or_else(|| Error::Custom("please give a inventory name".to_string()))?;
    let reason = Arc::new(Mutex::new(None::<Error>));

    let excutors = Job::load(job, inventory)?;
    for (hosts, tasks) in excutors {
        {
            let reason = reason.lock();
            if let Ok(ref reason) = reason {
                if let Some(ref e) = reason.deref() {
                    return Err(Error::Custom(e.to_string()));
                }
            }
        }
        let mut children = vec![];

        for (host, vars) in hosts {
            let host = host.clone();
            let vars = vars.clone();
            let tasks = tasks.clone();
            let reason = reason.clone();
            let inventory = inventory.to_string();
            children.push(
                thread::Builder::new()
                    .name(format!("{}-{}-{}", host, job, inventory))
                    .spawn(move || {
                        let reason = reason.clone();
                        for task in tasks {
                            info!("run {} on {}", task, host);
                            if let Err(e) = task.run(&inventory, &host, &vars) {
                                if let Ok(mut reason) = reason.lock() {
                                    *reason = Some(e);
                                }
                                return;
                            }
                        }
                    })?,
            );
        }
        for it in children {
            info!("waiting for thread finished...");
            let _ = it.join();
        }
    }

    info!("Done.");
    Ok(())
}
