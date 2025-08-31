use crate::game::GameState;
use crate::ui::cli;

pub trait UI {
    fn render(&mut self, game_state: &GameState);
    fn get_input(&mut self) -> (usize, usize);
    fn show_message(&mut self, message: &str);
    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct CliUI;

impl UI for CliUI {
    fn render(&mut self, game_state: &GameState) {
        cli::print_board(
            &game_state.players_board,
            &game_state.computers_board,
            &game_state.computers_ships_lifes
        );
    }
    
    fn get_input(&mut self) -> (usize, usize) {
        cli::get_position_input()
    }
    
    fn show_message(&mut self, message: &str) {
        println!("{}", message);
    }
    
    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}