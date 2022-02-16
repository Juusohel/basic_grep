use basic_grep::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with arguments provided: {}", err);
        process::exit(1)
    });

    if let Err(e) = basic_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
