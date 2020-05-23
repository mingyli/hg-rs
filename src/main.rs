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
        .subcommand(clap::SubCommand::with_name("debugindex"))
        .subcommand(clap::SubCommand::with_name("snap"))
        .subcommand(
            clap::SubCommand::with_name("debugdata")
                .arg(clap::Arg::with_name("revision").required(true)),
        )
        .get_matches();
    match matches.subcommand() {
        ("init", Some(_)) => command::init()?,
        ("snap", Some(_)) => command::snap()?,
        ("debugindex", Some(_)) => command::debug_index()?,
        ("debugdata", Some(matches)) => {
            command::debugdata(clap::value_t!(matches, "revision", u64)?)?
        }
        _ => unreachable!(),
    }
    Ok(())
}
