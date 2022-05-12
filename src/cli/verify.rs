use clap::{Arg, ArgMatches, Command};
use log::debug;

pub fn subcommand() -> Command<'static> {
    let contract_address_arg = Arg::new("address").index(1).required(true);
    Command::new("verify")
        .about("verify source code of deployed smart contracts")
        .arg(contract_address_arg)
}

pub fn run(matches: &ArgMatches) {
    debug!("Entering verify::run");
    let contract_address = matches.value_of("address").unwrap();
    debug!("contract address: {}", contract_address);
}
