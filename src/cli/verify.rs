use crate::cli::model::CompiledFile;
use crate::cli::term;
use crate::compiler::protostar;
use crate::starknet;
use anyhow::{Error, Result};
use clap::{arg, ArgMatches, Command};
use std::env;
use std::fs;

pub fn subcommand() -> Command<'static> {
    Command::new("verify")
        .about("verify source code of deployed smart contracts")
        .arg(
            arg!([address] "Address of the smart contract")
                .index(1)
                .required(true),
        )
        .arg(
            arg!(
                -n --name <CONTRACT_NAME> "Contract name"
            )
            .default_value("main")
            .required(false),
        )
        .arg(
            arg!(
                -p --projectdir <PROJECT_DIR> "Project root directory"
            )
            .default_value("")
            .required(false),
        )
        .arg(
            arg!(
                -b --builddir <BUILD_DIR> "Build directory"
            )
            .default_value("")
            .required(false),
        )
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    // Parse CLI arguments
    let contract_address = String::from(matches.value_of("address").unwrap());
    let contract_name = String::from(matches.value_of("name").unwrap());
    let mut project_dir = matches.value_of("projectdir").unwrap();
    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.into_os_string().into_string().unwrap();
    if project_dir.is_empty() {
        project_dir = &current_dir;
    }
    let mut build_dir = String::from(matches.value_of("builddir").unwrap());
    if build_dir.is_empty() {
        build_dir = format!("{}/build", project_dir);
    }

    // Compile contracts
    protostar::compile(String::from(project_dir), build_dir.clone())?;

    // Retrieve compiled contract file
    let compiled_contract_file_path = format!("{}/{}.json", build_dir, &contract_name);

    // Parse compiled contract file
    let compiled_contract_json_str =
        fs::read_to_string(compiled_contract_file_path).map_err(Error::msg)?;
    let compiled_file: CompiledFile = serde_json::from_str(&compiled_contract_json_str)?;

    // Get deployed contract code on the blockchain
    let deployed_code = starknet::get_code(&contract_address)?;

    // Compare the results
    if compiled_file.program.data.eq(&deployed_code.bytecode) {
        term::display_success("Source code matches deployed bytecode.");
    } else {
        term::display_red("Source code does not match deployed bytecode.");
    }

    Ok(())
}
