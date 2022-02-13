use std::{env, process};
use basic_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); //collecting commandline arguments to Vec
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem with arguments provided: {}", err);
        process::exit(1)
    });

    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = basic_grep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}


