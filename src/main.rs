use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //collecting commandline arguments to Vec
    let config = parse_config(&args);

    println!("Searching for {} in {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Error reading the file"); //reading file

    println!("Text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename}
}

