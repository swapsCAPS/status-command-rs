use std::{fs, process};

const cmd: &str = "upower -i /org/freedesktop/UPower/devices/battery_BAT0";
const BAT_0_UEVENT_PATH: &str = "/sys/class/power_supply/BAT0/uevent";

fn parse_supply_capacity (file_content: String) -> Result<u8, String> {
    let arr: Vec<&str> = file_content
        .lines()
        .filter(|line| { line.contains("POWER_SUPPLY_CAPACITY=") })
        .collect();

    if arr.len() != 1 { return Err(format!("Parsing went wrong found {} matching lines", arr.len())); }

    let string_val = arr[0].split("=").collect::<Vec<&str>>()[1];

    let val = match string_val.parse::<u8>() {
        Ok(v)  => Ok(v),
        Err(e) => Err(format!("Could not parse {}: {}", string_val, e))
    }?;

    if val > 100 {
        return Err(String::from("Value can not be more than 100"))
    };

    Ok(val)
}

fn main() {
    process::Command::new(cmd);

    let battery_status = fs::read_to_string(BAT_0_UEVENT_PATH).unwrap_or_else(|err| {
        println!("Could not read file: {}: {}", BAT_0_UEVENT_PATH, err);
        process::exit(1)
    });

    let int = parse_supply_capacity(battery_status).unwrap_or_else(|err| {
        println!("Could not parse battery capacity: {}", err);
        process::exit(1)
    });

    println!("int {}", int);
}
