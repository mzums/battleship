use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    crossterm::event::{self, Event, KeyCode},
};
use std::io::Stdout;

use crate::ui::main::UI;
use crate::game;

pub struct TuiUI {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub cursor_pos: (u16, u16),
    pub message: String,
}

impl UI for TuiUI {
    fn render(&mut self, game_state: &game::GameState) {
        self.terminal.draw(|f| {
            //tui::render_boards(f, game_state, self.cursor_pos, &self.message);
        }).unwrap();
    }
    
    fn get_input(&mut self) -> (usize, usize) {
        loop {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Up => {/* todo */},
                    KeyCode::Down => {/* todo */},
                    KeyCode::Enter => return (self.cursor_pos.0 as usize, self.cursor_pos.1 as usize),
                    _ => {}
                }
            }
            //self.render(game_state);
        }
    }
    
    fn show_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        ratatui::crossterm::terminal::disable_raw_mode()?;
        ratatui::crossterm::execute!(
            self.terminal.backend_mut(),
            ratatui::crossterm::terminal::LeaveAlternateScreen
        )?;
        Ok(())
    }
}