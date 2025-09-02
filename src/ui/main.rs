use crate::game::GameState;

pub trait UI {
    fn render(&mut self, game_state: &GameState);
    fn get_input(&mut self, game_state: &GameState) -> (usize, usize);
    fn show_message(&mut self, message: &str);
    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
