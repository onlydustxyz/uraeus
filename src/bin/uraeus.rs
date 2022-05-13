extern crate log;
use uraeus::cli::{self, term};

fn main() {
    env_logger::init();
    let result = cli::execute();
    match result {
        Ok(()) => {}
        Err(e) => term::display_error(&e.to_string()),
    }
}
