use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

use clap::{App, Arg};
use failure::Error;

use super::{env, errors::Result, models::Job};

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
                .help("Job name")
                .takes_value(true),
        )
        .get_matches();

    let name = matches
        .value_of("job")
        .ok_or_else(|| format_err!("please give a job name"))?;
    let reason = Arc::new(Mutex::new(None::<Error>));

    let excutors = Job::load(name)?;
    for (hosts, tasks) in excutors {
        {
            let reason = reason.lock();
            if let Ok(ref reason) = reason {
                if let Some(ref e) = reason.deref() {
                    return Err(format_err!("{}", e));
                }
            }
        }
        let mut children = vec![];

        for (host, vars) in hosts {
            let host = host.clone();
            let vars = vars.clone();
            let tasks = tasks.clone();
            let reason = reason.clone();
            children.push(
                thread::Builder::new()
                    .name(format!("{} - {}", host, name))
                    .spawn(move || {
                        let reason = reason.clone();
                        for task in tasks {
                            if let Err(e) = task.run(&host, &vars) {
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
            let _ = it.join();
        }
    }

    info!("Done.");
    Ok(())
}
