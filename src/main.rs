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


fn play<U: UI>(ui: &mut U, game_state: &mut game::GameState) {
    while !game::GameState::check_if_end(game_state) {
        loop {
            ui.show_message("Your turn!");
            let (row, col) = ui.get_input(game_state);
            game::GameState::hit(&mut game_state.computers_board, (row, col), &mut game_state.computers_ships_lifes);
            ui.render(game_state);

            if game_state.computers_board[row][col][0] != 2 {
                break;
            }
        }
        
        ui.show_message("\nOpponent's turn!");
        
        loop {
            let last_move: (usize, usize) = ai::computers_turn(&mut game_state.players_board, &mut game_state.players_ships_lifes, game_state.last_hit);
            
            if game_state.players_board[last_move.0][last_move.1][0] == 2 {
                game_state.last_hit = last_move;
            }
            ui.show_message(format!("Computer hit at ({}, {})!", last_move.0 + 1, last_move.1 + 1).as_str());
            
            ui.render(game_state);

            if game_state.players_board[last_move.0][last_move.1][0] != 2 {
                break;
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let tui = args.tui;

    let mut game_state = game::GameState::new();

    if tui {
        let mut ui = match tui::TuiUI::new() {
            Ok(ui) => ui,
            Err(e) => {
                eprintln!("Failed to initialize TUI: {}", e);
                return;
            }
        };
        
        ui.show_message("Welcome to Battleship! Use arrow keys to move, Enter to shoot");
        
        play(&mut ui, &mut game_state);
        
        if let Err(e) = ui.cleanup() {
            eprintln!("Error during TUI cleanup: {}", e);
        }
    } else {
        let mut ui = cli::CliUI;
        ui.render(&game_state);
        play(&mut ui, &mut game_state);
    }
}