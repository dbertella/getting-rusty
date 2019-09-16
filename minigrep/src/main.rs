use std::process;
use std::env;

use minigrep::Config; // how?

fn main() {
    let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Problem reading file: {}", e);
        process::exit(1);
    }
}
