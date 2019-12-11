use std::fs;
use std::{env};
use colored::*;

pub struct Config {
    pub val: u8,
}

struct Box<'a> {
    empty: &'a str,
    half:  &'a str,
    full:  &'a str,
}

const BOX: Box<'static> = Box {
    empty: "□",
    half:  "▥",
    full:  "▣",
};

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(String::from("Not enough args"));
        }

        let val = match args[1].parse::<u8>() {
            Ok(v)  => Ok(v),
            Err(e) => Err(format!("Could not parse {}: {}", args[1], e))
        }?;

        if val > 100 {
            return Err(String::from("Value can not be more than 100"))
        }

        Ok(Config { val })
    }
}

pub fn num_to_bars (val: u8) -> String {
    let mut bars    = Vec::new();

	let amount = val as f32 * (5.0 / 100.0);

	let rounded = (amount / 0.5).round() * 0.5;

    for _ in 0..rounded as u8 {
        bars.push(BOX.full);
    };

    if rounded % 1.0 == 0.5 {
        bars.push(BOX.half);
    };

    for _ in 0..5 - bars.len() {
        bars.push(BOX.empty);
    }

    let string: String = bars.into_iter().collect();

    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for i in 0..=100 {
            println!("{}: {}", i, num_to_bars(Config { val: i }));
        }
    }

}
