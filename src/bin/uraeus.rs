extern crate log;
use uraeus::cli::{self, term};

fn main() {
    env_logger::init();
    println!("GM from Uræus!");
    let result = cli::execute();
    match result {
        Ok(()) => term::display_success("Command executed with success."),
        Err(e) => term::display_error(&e.to_string()),
    }
}
