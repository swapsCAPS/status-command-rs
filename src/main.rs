use std::{env, process};
use status_command_rs::{ Config, num_to_bars };

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Invalid input: {}", err);
        process::exit(1)
    });

    num_to_bars(config);
}
