use rand::Rng;
use std::io;
use regex::Regex;
use std::cmp::max;
use rand::seq::SliceRandom;
use colored::Colorize;
use std::thread;
use std::time::Duration;

// 0 - empty
// 1 - ship
// 2 - hit
// 3 - missed


const MAX_SHIPS_LIFES: [i32; 6] = [0, 5, 4, 3, 3, 2];
const EMPTY1: [i32; 1] = [0];
const TRIED1: [i32; 2] = [0, 1];
const EMPTY2: [i32; 1] = [0];
const TRIED2: [i32; 3] = [0, 1, 3];


fn print_board(players_board: &[[[i32; 2]; 10]; 10], computers_board: &[[[i32; 2]; 10]; 10], computers_ships_lifes: &mut [i32; 6]) {
    println!("   A B C D E F G H I J         A B C D E F G H I J");
    for i in 0..10 {
        print_board_line(players_board, i, false, computers_ships_lifes);
        print!("     ");
        print_board_line(computers_board, i, true, computers_ships_lifes);
        println!();
    }
    println!("      Your board                Opponent's board");
}

fn print_board_line(board: &[[[i32; 2]; 10]; 10], i: usize, hidden: bool, computers_ships_lifes: &mut [i32; 6]) {
    print!("{} ", i);
    for j in 0..10 {
        print!("|");
        //print!("{}", board[i][j][0]);
        if board[i][j][0] == 0 || (hidden && board[i][j][0] == 1) {
            print!(" ");
        } else if board[i][j][0] == 1 || (hidden && board[i][j][0] == 2 && computers_ships_lifes[board[i][j][1] as usize] == 0) {
            print!("{}", "X".bold().blue());
        } else if board[i][j][0] == 2 {
            print!("{}", "#".bold().red());
        } else {
            print!("#");
        }
    }
    print!("|");
}

fn can_place_ship(board: &[[[i32; 2]; 10]; 10], x: usize, y: usize, length: usize, horizontal: bool, permitted1: &[i32], permitted2: &[i32]) -> bool {
    let (dx, dy) = if horizontal { (0, 1) } else { (1, 0) };

    for i in 0..length {
        let nx = x + i * dx;
        let ny = y + i * dy;


        if nx >= 10 || ny >= 10 || !permitted1.contains(&board[nx][ny][0]) {
            return false;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let sx = nx as isize + dx;
                let sy = ny as isize + dy;
                if sx >= 0 && sx < 10 && sy >= 0 && sy < 10 && !permitted2.contains(&board[sx as usize][sy as usize][0])
                {
                    return false;
                }
            }
        }
    }
    true
}

fn place_ship(board: &mut [[[i32; 2]; 10]; 10], length: usize, id: i32, ships_lifes: &mut [i32; 6]) {
    ships_lifes[id as usize] = length as i32;
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(0..10);
        let y = rng.gen_range(0..10);
        let horizontal = rng.gen_bool(0.5);
        if can_place_ship(board, x, y, length, horizontal, &EMPTY1, &EMPTY2) {
            if horizontal {
                for i in 0..length {
                    board[x][y + i][1] = id;
                    board[x][y + i][0] = 1;
                }
            } else {
                for i in 0..length {
                    board[x + i][y][1] = id;
                    board[x + i][y][0] = 1;
                }
            }
            break;
        }
    }
}

fn place_ships(board: &mut [[[i32; 2]; 10]; 10], ships_lifes: &mut [i32; 6]) {
    for i in 1..6 {
        place_ship(board,MAX_SHIPS_LIFES[i] as usize, i as i32, ships_lifes);
    }
}

fn did_win(ships_lifes: &mut [i32; 6]) -> bool{
    for i in ships_lifes {
        if *i != 0 {
            return false;
        }
    }
    return true;
}

fn check_if_end(players_ships_lifes: &mut [i32; 6], computers_ships_lifes: &mut [i32; 6]) -> bool {
    if did_win(players_ships_lifes) || did_win(computers_ships_lifes) {
        return true;
    }
    return false;
}

fn play(players_board: &mut [[[i32; 2]; 10]; 10], computers_board: &mut [[[i32; 2]; 10]; 10], players_ships_lifes: &mut [i32; 6], computers_ships_lifes: &mut [i32; 6]) {
    let mut last_hit: (usize, usize) = (10, 10);

    while !check_if_end(players_ships_lifes, computers_ships_lifes) {
        loop {
            println!("Your turn!");
            let (row, col) = get_position_input();
            hit(computers_board, (row, col), computers_ships_lifes);
            print_board(&players_board, &computers_board, computers_ships_lifes);
            if computers_board[row][col][0] != 2 {
                break;
            }
        }
        println!();
        loop {
            println!("Opponent's turn!");
            let last_move: (usize, usize) = computers_turn(players_board, players_ships_lifes, last_hit);
            if players_board[last_move.0][last_move.1][0] == 2 {
                last_hit = last_move;
            }
            thread::sleep(Duration::from_secs(1));
            print_board(&players_board, &computers_board, computers_ships_lifes);
            if players_board[last_move.0][last_move.1][0] != 2 {
                break;
            }
        }
        println!();
    }
}

fn create_heatmap(players_board: &mut [[[i32; 2]; 10]; 10], players_ships_lifes: &mut [i32; 6]) -> [[i32; 10]; 10] {
    let mut heatmap: [[i32; 10]; 10] = [[0; 10]; 10];

    for y in 0..10 {
        for x in 0..10 {
            for (index, &len) in MAX_SHIPS_LIFES.iter().enumerate() {
                if players_ships_lifes[index] == 0 {
                    //println!("{} empty", index);
                    continue;
                }
                for k in 0..2 {
                    if can_place_ship(&players_board, y, x, len as usize, k == 1, &TRIED1, &TRIED2) {
                        for i in 0..len {
                            if k == 1 {
                                heatmap[y][x + i as usize] += 1;
                            } else {
                                heatmap[y + i as usize][x] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    /*for i in 0..10 {
        for j in 0..10 {
            print!("{} ", heatmap[i][j]);
        }
        println!();
    }*/

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
    println!("Opponent's move: {}{}", (chosen_move.1 as u8 + 'A' as u8) as char, chosen_move.0);
    
    return chosen_move;
}

fn did_last_sink(players_board: &mut [[[i32; 2]; 10]; 10], last_hit: (usize, usize), players_ships_lifes: &mut [i32; 6]) -> bool {
    if players_ships_lifes[players_board[last_hit.0][last_hit.1][1] as usize] == 0 {
        return true;
    }
    return false;
}

fn continue_hitting_ship(players_board: &mut [[[i32; 2]; 10]; 10], last_hit: (usize, usize)) -> (usize, usize) {
    let (mut x, mut y): (usize, usize) = last_hit;

    let mut possibilities: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut rng = rand::thread_rng();
    possibilities.shuffle(&mut rng);

    for &(x2, y2) in &possibilities {
        let new_x: i32 = x as i32 + x2;
        let new_y: i32 = y as i32 + y2;
        if new_x < 0 || new_x >= 10 || new_y < 0 || new_y >= 10 {
            continue;
        }
        let new_x2 = x as i32 - x2;
        let new_y2 = y as i32 - y2;
        if players_board[new_x as usize][new_y as usize][0] == 2 {
            if new_x2 < 0 || new_x2 >= 10 || new_y2 < 0 || new_y2 >= 10 ||
                players_board[new_x2 as usize][new_y2 as usize][0] == 3 {
                loop {
                    x = (x as i32 + x2) as usize;
                    y = (y as i32 + y2) as usize;
                    
                    if players_board[x][y][0] == 0 || players_board[x][y][0] == 1 {
                        return (x, y);
                    }
                }
            }
            return (new_x2 as usize, new_y2 as usize);
        }
    }
    for &(x2, y2) in &possibilities {
        let new_x: i32 = x as i32 + x2;
        let new_y: i32 = y as i32 + y2;
        if new_x < 0 || new_x >= 10 || new_y < 0 || new_y >= 10 {
            continue;
        }
        if players_board[new_x as usize][new_y as usize][0] != 3 {
            return (new_x as usize, new_y as usize);
        }
    }
    return ((x as i32 + possibilities[0].0) as usize, (y as i32 + possibilities[0].1) as usize);
}

fn computers_turn(players_board: &mut [[[i32; 2]; 10]; 10], players_ships_lifes: &mut [i32; 6], last_hit: (usize, usize)) -> (usize, usize) {
    let chosen_move: (usize, usize);

    if last_hit == (10, 10) || did_last_sink(players_board, last_hit, players_ships_lifes) {
        let heatmap: [[i32; 10]; 10] = create_heatmap(players_board, players_ships_lifes);
        chosen_move = find_optimal_move(heatmap);
    } else {
        chosen_move = continue_hitting_ship(players_board, last_hit);
    }
    
    hit(players_board, chosen_move, players_ships_lifes);

    return chosen_move;
}

fn hit(board: &mut [[[i32; 2]; 10]; 10], chosen_move: (usize, usize), ships_lifes: &mut [i32; 6]) {
    let (row, col): (usize, usize) = chosen_move;
    if board[row][col][0] == 0 {
        println!("{}", "Missed!".red().bold());
        board[row][col][0] = 3;
    }
    else if board[row][col][0] == 2 || board[row][col][0] == 3 {
        println!("{}", "Already tried this one!".red().bold());
    } else if ships_lifes[board[row][col][1] as usize] == 1{
        println!("{}", "Hit and sunk!".red().bold());
        ships_lifes[board[row][col][1] as usize] -= 1;
        board[row][col][0] = 2;
    }
    else {
        println!("{}", "Hit!".red().bold());
        ships_lifes[board[row][col][1] as usize] -= 1;
        board[row][col][0] = 2;
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
    let mut players_board: [[[i32; 2]; 10]; 10] = [[[0; 2]; 10]; 10];
    let mut computers_board: [[[i32; 2]; 10]; 10] = [[[0; 2]; 10]; 10];
    let mut players_ships_lifes: [i32; 6] = [0, 5, 4, 3, 3, 2];
    let mut computers_ships_lifes: [i32; 6] = [0, 5, 4, 3, 3, 2];
    
    println!("Hello, world!");
    place_ships(&mut players_board, &mut players_ships_lifes);
    place_ships(&mut computers_board, &mut computers_ships_lifes);
    print_board(&players_board, &computers_board, &mut players_ships_lifes);

    play(&mut players_board, &mut computers_board, &mut players_ships_lifes, &mut computers_ships_lifes);
}
