use clap::Parser;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::Stdout;
use std::thread;
use std::time::Duration;

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
use colored::Colorize;

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


fn play(ui: &mut dyn UI, game_state: &mut game::GameState) -> Result<(), Box<dyn std::error::Error>> {
    while !game_state.check_if_end() {
        ui.show_message("Your turn!");
        let (row, col) = ui.get_input(game_state);
        let message = game::GameState::hit(&mut game_state.computers_board, (row, col), &mut game_state.computers_ships_lifes);
        ui.show_message(&message);
        ui.render(game_state);

        if game_state.did_win(&game_state.computers_ships_lifes) {
            break;
        }

        ui.show_message("Computer's turn!");
        let last_move = ai::computers_turn(&mut game_state.players_board, &mut game_state.players_ships_lifes, game_state.last_hit);
        
        if game_state.players_board[last_move.0][last_move.1][0] == 2 {
            game_state.last_hit = last_move;
        }
        
        thread::sleep(Duration::from_secs(1));
        ui.render(game_state);
    }
    
    Ok(())
}

fn main() {
    let args = Args::parse();
    let tui_mode = args.tui;

    let mut game_state = game::GameState::new();

    board::place_ships(&mut game_state.players_board, &mut game_state.players_ships_lifes);
    board::place_ships(&mut game_state.computers_board, &mut game_state.computers_ships_lifes);

    if tui_mode {
        let mut ui = match tui::TuiUI::new() {
            Ok(ui) => ui,
            Err(e) => {
                eprintln!("Failed to initialize TUI: {}", e);
                return;
            }
        };

        ui.render(&game_state);
        if let Err(e) = play(&mut ui, &mut game_state) {
            eprintln!("Error during game: {}", e);
        }
        
        if let Err(e) = ui.cleanup() {
            eprintln!("Failed to clean up TUI: {}", e);
        }
    } else {
        let mut ui = cli::CliUI;
        ui.render(&game_state);
        play(&mut ui, &mut game_state).expect("CLI game error");
    }

    if game_state.did_win(&game_state.players_ships_lifes) {
        if tui_mode {
            tui::UI::show_message(&mut game_state, "You lost! Computer wins!");
            thread::sleep(Duration::from_secs(2));
        } else {
            println!("{}", "You lost! Computer wins!".red().bold());
        }
    } else {
        if tui_mode {
        } else {
            println!("{}", "Congratulations! You win!".green().bold());
        }
    }
}