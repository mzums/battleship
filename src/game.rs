use crate::board::place_ships;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Player,
    Computer,
}

pub struct GameState {
    pub players_board: [[[i32; 2]; 10]; 10],
    pub computers_board: [[[i32; 2]; 10]; 10],
    pub players_ships_lifes: [i32; 6],
    pub computers_ships_lifes: [i32; 6],
    pub last_hit: (usize, usize),
    pub turn: Turn,
}

impl GameState {
    pub fn new() -> Self {
        let mut players_board = [[[0; 2]; 10]; 10];
        let mut computers_board = [[[0; 2]; 10]; 10];
        let mut players_ships_lifes = [0, 5, 4, 3, 3, 2];
        let mut computers_ships_lifes = [0, 5, 4, 3, 3, 2];
        
        place_ships(&mut players_board, &mut players_ships_lifes);
        place_ships(&mut computers_board, &mut computers_ships_lifes);
        
        Self {
            players_board,
            computers_board,
            players_ships_lifes,
            computers_ships_lifes,
            last_hit: (10, 10),
            turn: Turn::Player,
        }
    }
    
    pub fn did_win(&self, ships_lifes: &[i32; 6]) -> bool {
        ships_lifes.iter().skip(1).all(|&life| life == 0)
    }
    
    pub fn check_if_end(&self) -> bool {
        self.did_win(&self.players_ships_lifes) || self.did_win(&self.computers_ships_lifes)
    }
    
    pub fn hit(board: &mut [[[i32; 2]; 10]; 10], chosen_move: (usize, usize), ships_lifes: &mut [i32; 6]) -> String {
        let (row, col) = chosen_move;
        
        if board[row][col][0] == 0 {
            board[row][col][0] = 3;
            "\nMissed!".to_string()
        } else if board[row][col][0] == 2 || board[row][col][0] == 3 {
            "\nAlready tried this one!".to_string()
        } else if ships_lifes[board[row][col][1] as usize] == 1 {
            ships_lifes[board[row][col][1] as usize] -= 1;
            board[row][col][0] = 2;
            "\nHit and sunk!".to_string()
        } else {
            ships_lifes[board[row][col][1] as usize] -= 1;
            board[row][col][0] = 2;
            "\nHit!".to_string()
        }
    }
}