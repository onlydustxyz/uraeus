pub mod model;
pub mod term;
pub mod ui;
pub mod verify;

use anyhow::Result;
use clap::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn execute() -> Result<()> {
    let matches = Command::new("uraeus")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .subcommand(verify::subcommand())
        .subcommand(ui::subcommand())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("verify") {
        verify::run(matches)?
    }
    if let Some(matches) = matches.subcommand_matches("ui") {
        ui::run(matches)?
    }
    Ok(())
}
