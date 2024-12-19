use rand::Rng;
use std::io;
use regex::Regex;

// 0 - empty
// 1-5 - id
// 8 - missed
// 9 - hit


fn print_board(players_board: &[[i32; 10]; 10], computers_board: &[[i32; 10]; 10]) {
    println!("   A B C D E F G H I J         A B C D E F G H I J");
    for i in 0..10 {
        print_board_line(players_board, i);
        print!("     ");
        print_board_line(computers_board, i);
        println!();
    }
    println!("      Your board                Opponent's board");
}

fn print_board_line(board: &[[i32; 10]; 10], i: usize) {
    print!("{} ", i);
    for j in 0..10 {
        print!("|");
        if board[i][j] == 0 {
            print!(" ");
        } else {
            print!("X");
        }
    }
    print!("|");
}

fn can_place_ship(board: &[[i32; 10]; 10], x: usize, y: usize, length: usize, horizontal: bool) -> bool {
    let (dx, dy) = if horizontal { (0, 1) } else { (1, 0) };

    for i in 0..length {
        let nx = x + i * dx;
        let ny = y + i * dy;

        if nx >= 10 || ny >= 10 || board[nx][ny] != 0 {
            return false;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                let sx = nx as isize + dx;
                let sy = ny as isize + dy;
                if sx >= 0 && sx < 10 && sy >= 0 && sy < 10 && board[sx as usize][sy as usize] != 0 {
                    return false;
                }
            }
        }
    }
    true
}


fn place_ship(board: &mut [[i32; 10]; 10], length: usize, id: i32, ship_lifes: &mut [i32; 6]) {
    ship_lifes[id as usize] = length as i32;
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(0..10);
        let y = rng.gen_range(0..10);
        let horizontal = rng.gen_bool(0.5);
        if can_place_ship(board, x, y, length, horizontal) {
            if horizontal {
                for i in 0..length {
                    board[x][y + i] = id;
                }
            } else {
                for i in 0..length {
                    board[x + i][y] = id;
                }
            }
            break;
        }
    }
}

fn place_ships(board: &mut [[i32; 10]; 10], ship_lifes: &mut [i32; 6]) {
    place_ship(board, 5, 1, ship_lifes);
    place_ship(board, 4, 2, ship_lifes);
    place_ship(board, 3, 3, ship_lifes);
    place_ship(board, 3, 4, ship_lifes); 
    place_ship(board, 2, 5, ship_lifes);
}

fn play(players_board: &mut [[i32; 10]; 10], computers_board: &mut [[i32; 10]; 10], players_ship_lifes: &mut [i32; 6], computers_ship_lifes: &mut [i32; 6]) {
    println!("Your turn!");
    let (row, col) = get_position_input();
    hit(computers_board, (row, col), computers_ship_lifes);
    println!("Opponent's turn!");
}

fn hit(board: &mut [[i32; 10]; 10], (row, col): (usize, usize), ship_lifes: &mut [i32; 6]) {
    if board[col][row] == 0 {
        println!("You missed!");
        board[row][col] = 8;
    }
    else if board[col][row] >= 8 {
        println!("Already tried this one!");
    }
    else {
        println!("Hit!");
        ship_lifes[board[row][col] as usize] -= 1;
        board[row][col] = 9;
    }
}

fn get_position_input() -> (usize, usize) {
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

fn parse_position_input(position: &str) -> Option<(usize, usize)> {
    let re = Regex::new(r"^[a-jA-J]\d$").unwrap();
    if re.is_match(position) {
        let row = position.chars().nth(0)?.to_lowercase().next()? as usize - 'a' as usize;
        let col = position.chars().nth(1)?.to_digit(10)? as usize;
        Some((row, col))
    } else {
        None
    }
}

fn main() {
    println!("Hello, world!");
    let mut players_board: [[i32; 10]; 10] = [[0; 10]; 10];
    let mut computers_board: [[i32; 10]; 10] = [[0; 10]; 10];
    let mut players_ship_lifes: [i32; 6] = [0; 6];
    let mut computers_ship_lifes: [i32; 6] = [0; 6];

    place_ships(&mut players_board, &mut players_ship_lifes);
    place_ships(&mut computers_board, &mut computers_ship_lifes);
    print_board(&players_board, &computers_board);

    play(&mut players_board, &mut computers_board, &mut players_ship_lifes, &mut computers_ship_lifes);
}
