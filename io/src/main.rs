/*
 *
 * We are going to make our own grep
 * Example usage: io phrase file.txt
 * When running it in development phase, we use
 * cargo run -- phrase file.txt
 */

use std::env;
use std::process;

use io::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for {} in file {}\n----------------------\n",
        config.query, config.file_path
    );

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
