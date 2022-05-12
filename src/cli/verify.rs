use clap::{arg, ArgMatches, Command};
use execute::Execute;
use log::{debug, error};
use std::env;
use std::process::{self, Command as CommandLine};

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

pub fn run(matches: &ArgMatches) {
    debug!("Entering verify::run");
    let contract_address = matches.value_of("address").unwrap();
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
    debug!("contract address: {}", contract_address);
    debug!("project dir: {}", project_dir);
    debug!("build dir: {}", build_dir);

    let mut compile_command = CommandLine::new("protostar");
    compile_command.current_dir(project_dir);
    compile_command.arg("build");
    let output = compile_command.execute_output().unwrap();
    if let Some(exit_code) = output.status.code() {
        if exit_code != 0 {
            error!("Build command failed");
            process::exit(1);
        }
    } else {
        error!("Build command interrupted!");
        process::exit(1);
    }
}
