use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); //collecting commandline arguments to Vec
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {} in {}", query, filename)

}
