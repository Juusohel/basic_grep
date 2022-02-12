use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //collecting commandline arguments to Vec
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {} in {}", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("Error reading the file"); //reading file

    println!("Text: \n{}", contents);

}
