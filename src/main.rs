use anyhow::{Context, Result};
use clap::{self, value_t};

mod command;
mod dirstate;
mod manifest;
mod record;
mod repository;
mod revlog;

fn main() -> Result<()> {
    let matches = clap::App::new("hg-rs")
        .author("mingyli")
        .subcommand(clap::SubCommand::with_name("init").about("Initialize a Mercurial repository."))
        .subcommand(
            clap::SubCommand::with_name("debugindex")
                .arg(clap::Arg::with_name("file").required(false))
                .arg(clap::Arg::with_name("manifest").long("manifest")),
        )
        .subcommand(
            clap::SubCommand::with_name("snapshot")
                .arg(clap::Arg::with_name("file").required(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("debugdata")
                .arg(clap::Arg::with_name("revision").required(true))
                .arg(clap::Arg::with_name("file").required(false))
                .arg(
                    clap::Arg::with_name("manifest")
                        .long("manifest")
                        .required(false),
                ),
        )
        .subcommand(clap::SubCommand::with_name("debugdirstate"))
        .get_matches();
    match matches.subcommand() {
        ("init", Some(_)) => command::init()?,
        ("snapshot", Some(matches)) => command::snapshot(
            matches
                .value_of("file")
                .context("Failed to get file name.")?,
        )?,
        ("debugindex", Some(matches)) => {
            if matches.is_present("manifest") {
                command::debug_manifest_index()?
            } else {
                command::debug_index(
                    matches
                        .value_of("file")
                        .context("Failed to get file name.")?,
                )?
            }
        }
        ("debugdata", Some(matches)) => {
            if matches.is_present("manifest") {
                command::debug_manifest_data(clap::value_t!(matches, "revision", u32)?)?
            } else {
                command::debug_data(
                    matches
                        .value_of("file")
                        .context("Failed to get file name.")?,
                    clap::value_t!(matches, "revision", u32)?,
                )?
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
