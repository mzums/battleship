use colored::Colorize;
use std::thread;
use std::time::Duration;

mod ai;
mod board;
mod utils;
use utils::get_position_input;
use crate::ai::computers_turn;
use crate::board::print_board;
use crate::board::place_ships;

// 0 - empty
// 1 - ship
// 2 - hit
// 3 - missed

pub const MAX_SHIPS_LIFES: [i32; 6] = [0, 5, 4, 3, 3, 2];
pub const EMPTY1: [i32; 1] = [0];
pub const TRIED1: [i32; 2] = [0, 1];
pub const EMPTY2: [i32; 1] = [0];
pub const TRIED2: [i32; 3] = [0, 1, 3];


fn did_win(ships_lifes: &[i32; 6]) -> bool {
    ships_lifes.iter().all(|&life| life == 0)
}

fn check_if_end(players_ships_lifes: &[i32; 6], computers_ships_lifes: &[i32; 6]) -> bool {
    did_win(players_ships_lifes) || did_win(computers_ships_lifes)
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


pub fn hit(board: &mut [[[i32; 2]; 10]; 10], chosen_move: (usize, usize), ships_lifes: &mut [i32; 6]) {
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



fn main() {
    println!("{}", "\nHello in battle ship game!\n".blue().bold());

    let mut players_board: [[[i32; 2]; 10]; 10] = [[[0; 2]; 10]; 10];
    let mut computers_board: [[[i32; 2]; 10]; 10] = [[[0; 2]; 10]; 10];
    let mut players_ships_lifes: [i32; 6] = [0, 5, 4, 3, 3, 2];
    let mut computers_ships_lifes: [i32; 6] = [0, 5, 4, 3, 3, 2];
    
    place_ships(&mut players_board, &mut players_ships_lifes);
    place_ships(&mut computers_board, &mut computers_ships_lifes);
    print_board(&players_board, &computers_board, &mut players_ships_lifes);

    play(&mut players_board, &mut computers_board, &mut players_ships_lifes, &mut computers_ships_lifes);
}
