use anyhow::Result;
use clap::{self, value_t};

mod command;
mod record;
mod repository;
mod revlog;

fn main() -> Result<()> {
    let matches = clap::App::new("hg-rs")
        .author("mingyli")
        .subcommand(clap::SubCommand::with_name("init").about("Initialize a Mercurial repository."))
        .subcommand(
            clap::SubCommand::with_name("debugindex")
                .arg(clap::Arg::with_name("file").required(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("snap").arg(clap::Arg::with_name("file").required(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("debugdata")
                .arg(clap::Arg::with_name("file").required(true))
                .arg(clap::Arg::with_name("revision").required(true)),
        )
        .get_matches();
    match matches.subcommand() {
        ("init", Some(_)) => command::init()?,
        ("snap", Some(matches)) => command::snap(matches.value_of("file").unwrap())?,
        ("debugindex", Some(matches)) => command::debug_index(matches.value_of("file").unwrap())?,
        ("debugdata", Some(matches)) => command::debugdata(
            matches.value_of("file").unwrap(),
            clap::value_t!(matches, "revision", u32)?,
        )?,
        _ => unreachable!(),
    }
    Ok(())
}
