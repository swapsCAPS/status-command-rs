use std::{env};

pub struct Config {
    pub val: u8,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(String::from("Not enough args"));
        }

        let val = match args[1].parse::<u8>() {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("Could not parse {}: {}", args[1], e))
        }?;

        Ok(Config { val })
    }
}

pub fn num_to_bars (str_val: String) -> String {
    return String::from(str_val)
}
