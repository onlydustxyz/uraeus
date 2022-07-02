use crate::web::service;
use anyhow::Result;
use clap::{arg, ArgMatches, Command};
use open;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct UIConfig {
    pub project_dir: String,
    pub port: String,
    pub browser: bool,
}

pub fn subcommand() -> Command<'static> {
    Command::new("ui")
        .about("run with the web ui")
        .arg(
            arg!(
                -p --projectdir <PROJECT_DIR> "Project root directory"
            )
            .default_value("")
            .required(false),
        )
        .arg(
            arg!(
                --port <PORT> "change the UI port"
            )
            .default_value("7878")
            .required(false),
        )
        .arg(
            arg!(
                -o --open "open the browser from the CLI"
            )
            .takes_value(false)
            .required(false),
        )
}

fn parse_config(matches: &ArgMatches) -> Result<UIConfig> {
    // Parse CLI arguments
    let mut project_dir = matches.value_of("projectdir").unwrap();
    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.into_os_string().into_string().unwrap();
    if project_dir.is_empty() {
        project_dir = &current_dir;
    }
    let port = String::from(matches.value_of("port").unwrap());
    let browser = matches.is_present("open");
    Ok(UIConfig {
        project_dir: String::from(project_dir),
        port,
        browser,
    })
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let config = parse_config(matches)?;
    match config.port.parse::<i64>() {
        Ok(n) => {
            if n < 1025 || n > 65535 {
                println!("port number must be between 1024 and 65535");
                return Ok(());
            }
            if config.browser {
                open::that(format!("http://localhost:{}", n)).unwrap();
            }
            service(n, config.project_dir).unwrap();
        }
        Err(_e) => {
            println!("Port number must be between 1024 and 65535");
            return Ok(());
        }
    }
    Ok(())
}
