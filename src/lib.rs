use std::{env};
use colored::*;

pub struct Config {
    pub val: u8,
}

struct Box<'a> {
    empty: &'a str, // "□",
    half: &'a str, // "▨",
    full: &'a str, // "▣",
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

pub fn num_to_bars (config: Config) -> String {
    let mut bars    = Vec::new();
    let val = config.val as f32;
	println!("val {}", val);

	let amount = val * (5.0 / 100.0);
	println!("amount {}", amount);

	let rounded = (amount / 0.5).round() * 0.5;
	println!("rounded {}", rounded);

	// for _ in 0..amount_of_full_bars {
		// bars.push(BOX.full);
	// };

	// for _ in 0..amount_of_half_bars {
		// bars.push(BOX.half);
	// };

    for _ in 0..5 - bars.len() {
        bars.push(BOX.empty);
    }

    let string: String = bars.into_iter().collect();

    println!("{}", string);

    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String, num_to_bars(Config { val: 75 }))
    }

}
