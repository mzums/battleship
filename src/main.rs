use std::io;
use regex::Regex;

fn print_board(players_board: &[[i32; 10]; 10]) {
    println!("   A  B  C  D  E  F  G  H  I  J");
    for i in 0..10 {
        print!("{i} ");
        for j in 0..10 {
            print!("| ");
            print!("{}", players_board[i][j]);
        }
        println!("|");
    }
}

fn parse_position(position: &str) -> Option<(usize, usize)> {
    let re = Regex::new(r"^[a-jA-J]\d$").unwrap();
    if re.is_match(position) {
        let row = position.chars().nth(0).unwrap().to_lowercase().next().unwrap() as usize - 'a' as usize;
        let col = position.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
        Some((row, col))
    } else {
        None
    }
}

fn input_ships_pos() -> ((usize, usize), (usize, usize)) {
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

fn place_ship(players_board: &mut [[i32; 10]; 10], len: usize) {
    loop {
        println!("Where would you like to place your {} ship?", "X".repeat(len));
        println!("Pass chosen range e.g. A0 {}0", ('A' as u8 + ((len - 1) as u8)) as char);

        let ((r1_row, r1_col), (r2_row, r2_col)) = input_ships_pos();

        if r1_row == r2_row && (r1_col as isize - r2_col as isize).abs() == (len - 1) as isize {
            let start = r1_col.min(r2_col);
            for i in 0..len {
                players_board[start + i][r1_col] = 1;
            }
            println!("Ship placed horizontally.");
            break;
        } else if r1_col == r2_col && (r1_row as isize - r2_row as isize).abs() == (len - 1) as isize {
            let start = r1_row.min(r2_row);
            for i in 0..len {
                players_board[r1_row][start + i] = 1;
            }
            println!("Ship placed vertically.");
            break;
        } else {
            println!(
                "Invalid placement. Ensure the ship is aligned and has a length of {}.",
                len
            );
        }
    }
}


fn place_users_ships(players_board: &mut [[i32; 10]; 10]) {
    place_ship(players_board, 5);
    print_board(&players_board);
    place_ship(players_board, 4);
    print_board(&players_board);
    place_ship(players_board, 3);
    print_board(&players_board);
    place_ship(players_board, 3);
    print_board(&players_board);
    place_ship(players_board, 2);
    print_board(&players_board);
}

fn main() {
    println!("Hello, world!");
    let mut players_board: [[i32; 10]; 10] = [[0; 10]; 10];

    print_board(&players_board);
    println!();
    place_users_ships(&mut players_board);
}
