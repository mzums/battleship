use colored::Colorize;
use std::io;
use regex::Regex;

use crate::game;
use crate::ui::main::UI;


pub fn get_position_input() -> (usize, usize) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if let Some(position) = parse_position_input(input) {
            return position;
        }

        println!("Please enter exactly one value in the format letter + digit (e.g. a0).");
    }
}

pub fn parse_position_input(position: &str) -> Option<(usize, usize)> {
    let re = Regex::new(r"^[a-jA-J]\d$").unwrap();
    if re.is_match(position) {
        let col = position.chars().nth(0)?.to_lowercase().next()? as usize - 'a' as usize;
        let row = position.chars().nth(1)?.to_digit(10)? as usize;
        Some((row, col))
    } else {
        None
    }
}

pub fn print_board(players_board: &[[[i32; 2]; 10]; 10], computers_board: &[[[i32; 2]; 10]; 10], computers_ships_lifes: &[i32; 6]) {
    println!("   A B C D E F G H I J         A B C D E F G H I J");
    for i in 0..10 {
        print_board_line(players_board, i, false, computers_ships_lifes);
        print!("     ");
        print_board_line(computers_board, i, true, computers_ships_lifes);
        println!();
    }
    println!("      Your board                Opponent's board");
}

fn print_board_line(board: &[[[i32; 2]; 10]; 10], i: usize, hidden: bool, computers_ships_lifes: &[i32; 6]) {
    print!("{} ", i);
    for j in 0..10 {
        print!("|");
       if board[i][j][0] == 0 || (hidden && board[i][j][0] == 1) {
            print!(" ");
        } else if board[i][j][0] == 1 || (hidden && board[i][j][0] == 2 && computers_ships_lifes[board[i][j][1] as usize] == 0) {
            print!("{}", "X".bold().blue());
        } else if board[i][j][0] == 2 {
            print!("{}", "#".bold().red());
        } else {
            print!("#");
        }
    }
    print!("|");
}




struct CliUI;

impl UI for CliUI {
    fn render(&mut self, game_state: &game::GameState) {
        // Renderowanie tekstowe
        print_board(&game_state.players_board, &game_state.computers_board, &game_state.computers_ships_lifes);
    }
    
    fn get_input(&mut self) -> (usize, usize) {
        get_position_input()
    }
    
    fn show_message(&mut self, message: &str) {
        println!("{}", message);
    }

    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
