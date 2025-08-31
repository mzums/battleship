use colored::Colorize;
use std::thread;
use std::time::Duration;
use clap::Parser;

mod ai;
mod board;
mod ui {
    pub mod main;
    pub mod cli;
    pub mod tui;
}
mod game;

use crate::ui::cli;

pub const MAX_SHIPS_LIFES: [i32; 6] = [0, 5, 4, 3, 3, 2];
pub const EMPTY1: [i32; 1] = [0];
pub const TRIED1: [i32; 2] = [0, 1];
pub const EMPTY2: [i32; 1] = [0];
pub const TRIED2: [i32; 3] = [0, 1, 3];

#[derive(Parser)]
struct Args {
    #[clap(long)]
    tui: bool,

    #[clap(long)]
    cli: bool,
}

fn did_win(ships_lifes: &[i32; 6]) -> bool {
    ships_lifes.iter().all(|&life| life == 0)
}

fn check_if_end(players_ships_lifes: &[i32; 6], computers_ships_lifes: &[i32; 6]) -> bool {
    did_win(players_ships_lifes) || did_win(computers_ships_lifes)
}

fn play(players_board: &mut [[[i32; 2]; 10]; 10], computers_board: &mut [[[i32; 2]; 10]; 10], players_ships_lifes: &mut [i32; 6], computers_ships_lifes: &mut [i32; 6], tui: bool) {
    let mut last_hit: (usize, usize) = (10, 10);

    while !check_if_end(players_ships_lifes, computers_ships_lifes) {
        loop {
            if !tui {
                println!("Your turn!");
            }
            
            let (row, col) = if tui {
                //tui::main::get_position_input(players_board, computers_board, computers_ships_lifes)
                (0, 0)
            } else {
                cli::get_position_input()
            };
            
            hit(computers_board, (row, col), computers_ships_lifes, tui);
            
            if !tui {
                cli::print_board(&players_board, &computers_board, computers_ships_lifes);
            }
            
            if computers_board[row][col][0] != 2 {
                break;
            }
        }
        
        if !tui {
            println!();
            println!("Opponent's turn!");
        }
        
        loop {
            let last_move: (usize, usize) = ai::computers_turn(players_board, players_ships_lifes, last_hit, tui);
            
            if players_board[last_move.0][last_move.1][0] == 2 {
                last_hit = last_move;
            }
            
            if tui {
                //tui::render(players_board, computers_board, players_ships_lifes, computers_ships_lifes, "Computer's turn");
                thread::sleep(Duration::from_millis(500));
            } else {
                thread::sleep(Duration::from_secs(1));
                cli::print_board(&players_board, &computers_board, computers_ships_lifes);
            }
            
            if players_board[last_move.0][last_move.1][0] != 2 {
                break;
            }
        }
        
        if !tui {
            println!();
        }
    }
}

pub fn hit(board: &mut [[[i32; 2]; 10]; 10], chosen_move: (usize, usize), ships_lifes: &mut [i32; 6], tui: bool) {
    let (row, col): (usize, usize) = chosen_move;
    let msg: String;
    
    if board[row][col][0] == 0 {
        board[row][col][0] = 3;
        msg = format!("{}", "Missed!".red().bold());
    } else if board[row][col][0] == 2 || board[row][col][0] == 3 {
        msg = format!("{}", "Already tried this one!".red().bold());
    } else if ships_lifes[board[row][col][1] as usize] == 1 {
        msg = format!("{}", "Hit and sunk!".red().bold());
        ships_lifes[board[row][col][1] as usize] -= 1;
        board[row][col][0] = 2;
    } else {
        msg = format!("{}", "Hit!".red().bold());
        ships_lifes[board[row][col][1] as usize] -= 1;
        board[row][col][0] = 2;
    }
    
    if !tui {
        println!("{}", msg);
    } else {
        //tui::set_message(msg);
    }
}

fn main() {
    let args = Args::parse();
    let tui = args.tui;
    
    if !tui {
        println!("{}", "\nHello in battle ship game!\n".blue().bold());
    }

    let mut players_board: [[[i32; 2]; 10]; 10] = [[[0; 2]; 10]; 10];
    let mut computers_board: [[[i32; 2]; 10]; 10] = [[[0; 2]; 10]; 10];
    let mut players_ships_lifes: [i32; 6] = [0, 5, 4, 3, 3, 2];
    let mut computers_ships_lifes: [i32; 6] = [0, 5, 4, 3, 3, 2];
    
    board::place_ships(&mut players_board, &mut players_ships_lifes);
    board::place_ships(&mut computers_board, &mut computers_ships_lifes);
    
    if tui {
        //tui::init().expect("Failed to initialize TUI");
        //tui::render(&players_board, &computers_board, &players_ships_lifes, &computers_ships_lifes, "Welcome to Battleship!");
    } else {
        cli::print_board(&players_board, &computers_board, &mut players_ships_lifes);
    }

    play(&mut players_board, &mut computers_board, &mut players_ships_lifes, &mut computers_ships_lifes, tui);
    
    if tui {
        //tui::cleanup().expect("Failed to cleanup TUI");
    }
    
    if did_win(&players_ships_lifes) {
        if tui {
            //tui::main::show_message("You lost! Computer wins!");
        } else {
            println!("{}", "You lost! Computer wins!".red().bold());
        }
    } else {
        if tui {
            //tui::main::show_message("Congratulations! You win!");
        } else {
            println!("{}", "Congratulations! You win!".green().bold());
        }
    }
}