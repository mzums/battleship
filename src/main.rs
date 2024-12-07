use std::io;
use regex::Regex;

fn print_board() {
    println!("   A  B  C  D  E  F  G  H  I  J");
    for i in 0..10 {
        print!("{i} ");
        for _ in 0..10 {
            print!("|  ");
        }
        println!("|");
    }
}

fn parse_position(position: &str) -> Option<(char, u8)> {
    let re = Regex::new(r"^[a-zA-Z]\d$").unwrap();
    if re.is_match(position) {
        let row = position.chars().nth(0).unwrap().to_lowercase().next().unwrap();
        let col = position.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
        Some((row, col))
    } else {
        None
    }
}

fn input_ships_pos() -> ((char, u8), (char, u8)) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");

        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() == 2 {
            if let (Some(r1), Some(r2)) = (parse_position(parts[0]), parse_position(parts[1])) {
                return (r1, r2);
            }
        }

        println!("Please enter exactly two values, each in the format letter + digit (e.g. a0 b1).");
    }
}

fn place_ship(len: i8) {
    println!("Where would you like to place your {} ship?", "X".repeat(len as usize));
    println!("Pass chosen range e.g. A0 {}0", ('A' as u8 + ((len-1) as u8)) as char);

    let ((r1_row, r1_col), (r2_row, r2_col)) = input_ships_pos();
    print!("{r1_row}");

    if (r1_row as u8 - r2_row as u8 == (len-1) as u8 && r1_col == r2_col) || (r1_col as u8 - r2_col as u8 == (len-1) as u8 && r1_row == r2_row) {
        return;
    }
    
}

fn place_users_ships() {
    place_ship(5);
}

fn main() {
    println!("Hello, world!");
    let _players_board: [[i32; 10]; 10] = [[0; 10]; 10];

    print_board();
    println!();
    place_users_ships();
}
