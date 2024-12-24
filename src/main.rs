use rand::Rng;
use std::io;
use regex::Regex;
use std::cmp::max;

// 0 - empty
// 1-5 - id
// 8 - missed
// 9 - hit


const MAX_SHIP_LIFES: [i32; 6] = [0, 5, 4, 3, 3, 2];
const EMPTY: [i32; 1] = [0];
const TRIED: [i32; 6] = [0, 1, 2, 3, 4, 5];
const EMPTY2: [i32; 3] = [0, 8, 9];
const TRIED2: [i32; 7] = [0, 1, 2, 3, 4, 5, 8];


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
        } else if (1..6).contains(&board[i][j]) {
            print!("X");
        } else {
            print!("#");
        }
    }
    print!("|");
}

fn can_place_ship(board: &[[i32; 10]; 10], x: usize, y: usize, length: usize, horizontal: bool, permitted: &[i32], permitted2: &[i32]) -> bool {
    let (dx, dy) = if horizontal { (0, 1) } else { (1, 0) };

    for i in 0..length {
        let nx = x + i * dx;
        let ny = y + i * dy;


        if nx >= 10 || ny >= 10 || !permitted.contains(&board[nx][ny]) {
            return false;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                let sx = nx as isize + dx;
                let sy = ny as isize + dy;
                if sx >= 0 && sx < 10 && sy >= 0 && sy < 10 && !permitted2.contains(&board[sx as usize][sy as usize])
                {
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
        if can_place_ship(board, x, y, length, horizontal, &EMPTY, &EMPTY2) {
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
    for i in 1..6 {
        place_ship(board,MAX_SHIP_LIFES[i] as usize, i as i32, ship_lifes,);
    }
}

fn play(players_board: &mut [[i32; 10]; 10], computers_board: &mut [[i32; 10]; 10], players_ship_lifes: &mut [i32; 6], computers_ship_lifes: &mut [i32; 6]) {
    println!("Your turn!");
    let (row, col) = get_position_input();
    hit(computers_board, (row, col), computers_ship_lifes);
    println!("Opponent's turn!");
    computers_turn(players_board, players_ship_lifes);
    print_board(&players_board, &computers_board);

    println!("Your turn!");
    let (row, col) = get_position_input();
    hit(computers_board, (row, col), computers_ship_lifes);
    println!("Opponent's turn!");
    computers_turn(players_board, players_ship_lifes);
    print_board(&players_board, &computers_board);

    println!("Your turn!");
    let (row, col) = get_position_input();
    hit(computers_board, (row, col), computers_ship_lifes);
    println!("Opponent's turn!");
    computers_turn(players_board, players_ship_lifes);
    print_board(&players_board, &computers_board);
}

fn create_heatmap(players_board: &mut [[i32; 10]; 10]) -> [[i32; 10]; 10] {
    let mut heatmap: [[i32; 10]; 10] = [[0; 10]; 10];

    for y in 0..10 {
        for x in 0..10 {
            for len in &MAX_SHIP_LIFES[1..] {
                for k in 0..2 {
                    if can_place_ship(&players_board, x, y, *len as usize, k == 1, &TRIED, &TRIED2) {
                        for i in 0..*len {
                            if k == 1 {
                                heatmap[x][y + i as usize] += 1;
                            } else {
                                heatmap[x + i as usize][y] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    for i in 0..10 {
        for j in 0..10 {
            print!("{} ", heatmap[i][j]);
        }
        println!();
    }

    return heatmap;
}

fn find_optimal_move(heatmap: [[i32; 10]; 10]) -> (usize, usize) {
    let mut max_value = 0;
    let mut moves: Vec<(usize, usize)> = Vec::new();
    let mut rng = rand::thread_rng();

    for x in 0..10 {
        for y in 0..10 {
            max_value = max(max_value, heatmap[x][y]);
        }
    }
    for x in 0..10 {
        for y in 0..10 {
            if heatmap[x][y] == max_value {
                moves.push((x, y));
            }
        }
    }
    let idx = rng.gen_range(0..moves.len());
    let chosen_move: (usize, usize) = moves[idx];
    println!("{:?}", chosen_move);
    
    return chosen_move;
}

fn computers_turn(players_board: &mut [[i32; 10]; 10], players_ship_lifes: &mut [i32; 6]) {
    let heatmap: [[i32; 10]; 10] = create_heatmap(players_board);
    let chosen_move: (usize, usize) = find_optimal_move(heatmap);
    hit(players_board, chosen_move, players_ship_lifes);
}

fn hit(board: &mut [[i32; 10]; 10], (row, col): (usize, usize), ship_lifes: &mut [i32; 6]) {
    if board[row][col] == 0 {
        println!("Missed!");
        board[row][col] = 8;
    }
    else if board[row][col] >= 8 {
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
        let col = position.chars().nth(0)?.to_lowercase().next()? as usize - 'a' as usize;
        let row = position.chars().nth(1)?.to_digit(10)? as usize;
        Some((row, col))
    } else {
        None
    }
}

fn main() {
    let mut players_board: [[i32; 10]; 10] = [[0; 10]; 10];
    let mut computers_board: [[i32; 10]; 10] = [[0; 10]; 10];
    let mut players_ship_lifes: [i32; 6] = [0; 6];
    let mut computers_ship_lifes: [i32; 6] = [0; 6];
    
    place_ships(&mut players_board, &mut players_ship_lifes);
    println!("Hello, world!");
    place_ships(&mut computers_board, &mut computers_ship_lifes);
    print_board(&players_board, &computers_board);

    play(&mut players_board, &mut computers_board, &mut players_ship_lifes, &mut computers_ship_lifes);
    
}
