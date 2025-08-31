use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Paragraph},
    Frame, style::{Style, Color}, text::{Text, Line}, layout::Alignment
};
use std::io::Stdout;
use crate::ui::main::UI;
use crate::game::GameState;
use ratatui::crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};

pub struct TuiUI {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub cursor_pos: (u16, u16),
    pub message: String,
}

impl TuiUI {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self {
            terminal,
            cursor_pos: (0, 0),
            message: String::new(),
        })
    }
}

impl UI for TuiUI {
    fn render(&mut self, game_state: &GameState) {
        let message = &self.message;
        let cursor_pos = self.cursor_pos;
        
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(10),
                        Constraint::Length(3),
                    ].as_ref()
                )
                .split(f.size());

            let title = Paragraph::new("Battleship Game")
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center);
            f.render_widget(title, chunks[0]);

            let board_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            let player_block = Block::default()
                .title("Your Board")
                .borders(Borders::ALL);
            f.render_widget(player_block, board_chunks[0]);
            
            let computer_block = Block::default()
                .title("Computer's Board")
                .borders(Borders::ALL);
            f.render_widget(computer_block, board_chunks[1]);

            let message_paragraph = Paragraph::new(message.clone())
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center);
            f.render_widget(message_paragraph, chunks[2]);
        }).expect("Failed to draw UI");
    }

    fn get_input(&mut self, game_state: &GameState) -> (usize, usize) {
        loop {
            if let Event::Key(key) = event::read().expect("Failed to read event") {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            self.cleanup().expect("Cleanup failed");
                            std::process::exit(0);
                        },
                        KeyCode::Up if self.cursor_pos.1 > 0 => {
                            self.cursor_pos.1 -= 1;
                            self.render(game_state);
                        },
                        KeyCode::Down if self.cursor_pos.1 < 9 => {
                            self.cursor_pos.1 += 1;
                            self.render(game_state);
                        },
                        KeyCode::Left if self.cursor_pos.0 > 0 => {
                            self.cursor_pos.0 -= 1;
                            self.render(game_state);
                        },
                        KeyCode::Right if self.cursor_pos.0 < 9 => {
                            self.cursor_pos.0 += 1;
                            self.render(game_state);
                        },
                        KeyCode::Enter => {
                            return (self.cursor_pos.0 as usize, self.cursor_pos.1 as usize);
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    fn show_message(&mut self, message: &str) {
        self.message = message.to_string();
        self.render(&self);
    }

    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen
        )?;
        Ok(())
    }
}
``