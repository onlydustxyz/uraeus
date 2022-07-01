use crate::cli::web::service;
use anyhow::Result;
use clap::{ArgMatches, Command};

pub fn subcommand() -> Command<'static> {
    Command::new("ui").about("run the ui")
}

pub fn run(_matches: &ArgMatches) -> Result<()> {
    service().unwrap();
    Ok(())
}
