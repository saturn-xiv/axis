use clap::{App, Arg};

use super::{env, errors::Result, orm};

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
    orm::open()?;

    // for it in env::Job::load(job)? {
    //     it.run(inventory)?;
    // }
    Ok(())
}
