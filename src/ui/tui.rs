use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction, Rect},
    style::{Style, Color, Modifier},
    text::{Text, Span},
    Terminal,
    Frame
};
use ratatui::crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use std::io::{Stdout, stdout, Write};

use crate::ui::main::UI;
use crate::game;

pub struct TuiUI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    message: String,
    cursor_pos: (u16, u16),
    should_quit: bool,
}

impl TuiUI {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        stdout.flush()?;
        
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self {
            terminal,
            message: String::from("Welcome to Battleship! Use arrow keys to move, Enter to shoot"),
            cursor_pos: (0, 0),
            should_quit: false,
        })
    }
    
    fn render_boards(f: &mut Frame, game_state: &game::GameState, message: &str, cursor_pos: (u16, u16)) {
        let size = f.area();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(12),
                Constraint::Length(3),
            ].as_ref())
            .split(size);
            
        let title = Paragraph::new("Battleship Game")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, chunks[0]);
        
        let board_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);
            
        let player_block = Block::default()
            .title(" Your Board ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue));
        f.render_widget(player_block, board_chunks[0]);
        
        let computer_block = Block::default()
            .title(" Opponent's Board ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red));
        f.render_widget(computer_block, board_chunks[1]);
        
        Self::draw_board(f, game_state, board_chunks[0], true);
        Self::draw_board(f, game_state, board_chunks[1], false);
        
        if cursor_pos.0 < 10 && cursor_pos.1 < 10 {
            let cell_x = board_chunks[1].x + 2 + cursor_pos.1 * 2;
            let cell_y = board_chunks[1].y + 1 + cursor_pos.0;
            f.set_cursor_position((cell_x, cell_y));
        }
        
        let message_para = Paragraph::new(Text::from(message.to_string()))
            .style(Style::default().fg(Color::Green))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(message_para, chunks[2]);
    }
    
    fn draw_board(f: &mut Frame, game_state: &game::GameState, area: Rect, is_player: bool) {
        let inner = Rect {
            x: area.x + 5,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };
        
        for col in 0..10 {
            let label = format!("{}", (b'A' + col as u8) as char);
            let x = inner.x + 2 + col * 2;
            let y = inner.y;
            f.render_widget(
                Span::styled(label, Style::default().fg(Color::Yellow)),
                Rect::new(x, y, 1, 1)
            );
        }
        
        for row in 0..10 {
            let label = format!("{}", row);
            let x = inner.x;
            let y = inner.y + 1 + row;
            f.render_widget(
                Span::styled(label, Style::default().fg(Color::Yellow)),
                Rect::new(x, y, 1, 1)
            );
        }
        
        for row in 0..10 {
            for col in 0..10 {
                let board = if is_player {
                    &game_state.players_board
                } else {
                    &game_state.computers_board
                };
                
                let cell = board[row][col];
                let x = inner.x + 2 + col as u16 * 2;
                let y = inner.y + 1 + row as u16;
                
                let symbol = if cell[0] == 0 {
                    " " // Water
                } else if cell[0] == 1 {
                    if is_player { "S" } else { "~" }
                } else if cell[0] == 2 {
                    "X"
                } else {
                    "M"
                };
                
                let style = if cell[0] == 0 {
                    Style::default().fg(Color::Blue)
                } else if cell[0] == 1 {
                    Style::default().fg(Color::Green)
                } else if cell[0] == 2 {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::White)
                };
                
                f.render_widget(
                    Span::styled(symbol, style),
                    Rect::new(x, y, 1, 1)
                );
            }
        }
    }
    
    fn handle_event(&mut self) -> Result<Option<(usize, usize)>, Box<dyn std::error::Error>> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            self.should_quit = true;
                            return Ok(None);
                        }
                        KeyCode::Up if self.cursor_pos.0 > 0 => {
                            self.cursor_pos.0 -= 1;
                        }
                        KeyCode::Down if self.cursor_pos.0 < 9 => {
                            self.cursor_pos.0 += 1;
                        }
                        KeyCode::Left if self.cursor_pos.1 > 0 => {
                            self.cursor_pos.1 -= 1;
                        }
                        KeyCode::Right if self.cursor_pos.1 < 9 => {
                            self.cursor_pos.1 += 1;
                        }
                        KeyCode::Enter => {
                            return Ok(Some((self.cursor_pos.0 as usize, self.cursor_pos.1 as usize)));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(None)
    }
}

impl UI for TuiUI {
    fn render(&mut self, game_state: &game::GameState) {
        let message = self.message.clone();
        let cursor_pos = self.cursor_pos;
        
        self.terminal.draw(|f| {
            Self::render_boards(f, game_state, &message, cursor_pos);
        }).unwrap();
    }

    fn get_input(&mut self, game_state: &game::GameState) -> (usize, usize) {
        loop {
            self.render(game_state);
            
            match self.handle_event() {
                Ok(Some(pos)) => return pos,
                Ok(None) if self.should_quit => {
                    std::process::exit(0);
                }
                Err(e) => {
                    self.message = format!("Error: {}", e);
                }
                _ => {}
            }
        }
    }
    
    fn show_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}