use std::{env, process};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //collecting commandline arguments to Vec
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem with arguments provided: {}", err);
        process::exit(1)
    });

    println!("Searching for {} in {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Error reading the file"); //reading file

    println!("Text: \n{}", contents);
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


