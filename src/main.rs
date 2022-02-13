use std::{env, process};
use std::error::Error;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //collecting commandline arguments to Vec
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem with arguments provided: {}", err);
        process::exit(1)
    });

    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Text: \n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename})
    }

}


