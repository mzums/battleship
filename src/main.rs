use colored::Colorize;
use clap::Parser;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    crossterm::event::{self, Event, KeyCode},
};
use std::io::Stdout;

mod ai;
mod board;
mod ui {
    pub mod main;
    pub mod cli;
    pub mod tui;
}
mod game;

use crate::ui::cli;
use crate::ui::tui;
use crate::ui::main::UI;

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


fn play<U: UI>(ui: &mut U, players_board: &mut [[[i32; 2]; 10]; 10], computers_board: &mut [[[i32; 2]; 10]; 10], players_ships_lifes: &mut [i32; 6], computers_ships_lifes: &mut [i32; 6], game_state: &mut game::GameState) {
    let mut last_hit: (usize, usize) = (10, 10);

    while !game::GameState::check_if_end(game_state) {
        loop {
            ui.show_message("Your turn!");
            let (row, col) = ui.get_input();
            
            game::GameState::hit(game_state, computers_board, (row, col), computers_ships_lifes);
            
            ui.render(game_state);

            
            if computers_board[row][col][0] != 2 {
                break;
            }
        }
        
        ui.show_message("\nOpponent's turn!");
        
        loop {
            let last_move: (usize, usize) = ai::computers_turn(players_board, players_ships_lifes, last_hit);
            
            if players_board[last_move.0][last_move.1][0] == 2 {
                last_hit = last_move;
            }
            
            ui.render(game_state);
            
            if players_board[last_move.0][last_move.1][0] != 2 {
                break;
            }
        }
    }
}

pub fn hit(board: &mut [[[i32; 2]; 10]; 10], chosen_move: (usize, usize), ships_lifes: &mut [i32; 6]) -> String{
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

    return msg;
}

fn main() {
    let args = Args::parse();
    let tui = args.tui;
    
    
    let mut players_board: [[[i32; 2]; 10]; 10] = [[[0; 2]; 10]; 10];
    let mut computers_board: [[[i32; 2]; 10]; 10] = [[[0; 2]; 10]; 10];
    let mut players_ships_lifes: [i32; 6] = [0, 5, 4, 3, 3, 2];
    let mut computers_ships_lifes: [i32; 6] = [0, 5, 4, 3, 3, 2];
    
    board::place_ships(&mut players_board, &mut players_ships_lifes);
    board::place_ships(&mut computers_board, &mut computers_ships_lifes);

    let mut game_state = game::GameState::new();

    if tui {
        //tui::init().expect("Failed to initialize TUI");
        //tui::render(&players_board, &computers_board, &players_ships_lifes, &computers_ships_lifes, "Welcome to Battleship!");
        let terminal_result = Terminal::new(CrosstermBackend::new(std::io::stdout()));
        let terminal: Terminal<CrosstermBackend<Stdout>> = match terminal_result {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to initialize TUI: {}", e);
                return;
            }
        };
        let mut ui = tui::TuiUI {
            terminal,
            cursor_pos: (0, 0),
            message: String::new(),
        };
        ui.render(&game_state);
        play(&mut ui, &mut players_board, &mut computers_board, &mut players_ships_lifes, &mut computers_ships_lifes, &mut game_state);
    } else {
        let mut ui = cli::CliUI;
        ui.render(&game_state);
        play(&mut ui, &mut players_board, &mut computers_board, &mut players_ships_lifes, &mut computers_ships_lifes, &mut game_state);
        cli::print_board(&players_board, &computers_board, &mut players_ships_lifes);
    }

    /*if did_win(&players_ships_lifes) {
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
    }*/
}