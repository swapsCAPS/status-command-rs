// use colored::*; // Ahw I do want this tho : )

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

pub fn num_to_bars (val: u8, amount_of_bars: u8) -> String {
    let mut bars    = Vec::new();

	let amount = val as f32 * (amount_of_bars as f32 / 100.0);

	let rounded = (amount / 0.5).round() * 0.5;

    for _ in 0..rounded as u8 {
        bars.push(BOX.full);
    };

    if rounded % 1.0 == 0.5 {
        bars.push(BOX.half);
    };

    for _ in 0..amount_of_bars - bars.len() as u8 {
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
            println!("{}: {}", i, num_to_bars(i, 10));
        }
    }

}
