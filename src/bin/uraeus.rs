extern crate log;
use uraeus::cli;

fn main() {
    env_logger::init();
    println!("GM from Uræus!");
    cli::execute()
}
