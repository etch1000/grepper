use grepper::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grepper::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
