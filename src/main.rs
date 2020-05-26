use anyhow::{Context, Result};
use clap::{self, value_t};

mod changeset;
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
                .arg(clap::Arg::with_name("file"))
                .arg(clap::Arg::with_name("changelog").long("changelog"))
                .arg(clap::Arg::with_name("manifest").long("manifest")),
        )
        .subcommand(
            clap::SubCommand::with_name("commit").arg(
                clap::Arg::with_name("message")
                    .short("m")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .subcommand(
            clap::SubCommand::with_name("snapshot")
                .arg(clap::Arg::with_name("file").required(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("debugdata")
                .arg(clap::Arg::with_name("revision").required(true))
                .arg(clap::Arg::with_name("file"))
                .arg(clap::Arg::with_name("changelog").long("changelog"))
                .arg(clap::Arg::with_name("manifest").long("manifest")),
        )
        .subcommand(clap::SubCommand::with_name("debugdirstate"))
        .get_matches();
    match matches.subcommand() {
        ("init", Some(_)) => command::init()?,
        ("commit", Some(matches)) => command::commit(
            matches
                .value_of("message")
                .context("Failed to get commit message.")?,
        )?,
        ("snapshot", Some(matches)) => command::snapshot(
            matches
                .value_of("file")
                .context("Failed to get file name.")?,
        )?,
        ("debugindex", Some(matches)) => {
            if matches.is_present("manifest") {
                command::debug_manifest_index()?
            } else if matches.is_present("changelog") {
                command::debug_changelog_index()?
            } else {
                command::debug_index(
                    matches
                        .value_of("file")
                        .context("Failed to get file name.")?,
                )?
            }
        }
        ("debugdata", Some(matches)) => {
            let rev = clap::value_t!(matches, "revision", u32)?;
            if matches.is_present("manifest") {
                command::debug_manifest_data(rev)?
            } else if matches.is_present("changelog") {
                command::debug_changelog_data(rev)?
            } else {
                command::debug_data(
                    matches
                        .value_of("file")
                        .context("Failed to get file name.")?,
                    rev,
                )?
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
