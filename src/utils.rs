use std::io;
use regex::Regex;


pub fn get_position_input() -> (usize, usize) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if let Some(position) = parse_position_input(input) {
            return position;
        }

        println!("Please enter exactly one value in the format letter + digit (e.g. a0).");
    }
}

pub fn parse_position_input(position: &str) -> Option<(usize, usize)> {
    let re = Regex::new(r"^[a-jA-J]\d$").unwrap();
    if re.is_match(position) {
        let col = position.chars().nth(0)?.to_lowercase().next()? as usize - 'a' as usize;
        let row = position.chars().nth(1)?.to_digit(10)? as usize;
        Some((row, col))
    } else {
        None
    }
}