use rand::Rng;
use std::cmp::max;
use rand::seq::SliceRandom;

use crate::{MAX_SHIPS_LIFES, TRIED1, TRIED2};
use crate::board::can_place_ship;
use crate::game;


fn go_same_direction(players_board: &mut [[[i32; 2]; 10]; 10], (mut x, mut y): (usize, usize), possibilities: [(i32, i32); 4]) -> (usize, usize) {
    for &(x2, y2) in &possibilities {
        let new_x: i32 = x as i32 + x2;
        let new_y: i32 = y as i32 + y2;
        if new_x < 0 || new_x >= 10 || new_y < 0 || new_y >= 10 {
            continue;
        }
        let new_x2 = x as i32 - x2;
        let new_y2 = y as i32 - y2;
        if players_board[new_x as usize][new_y as usize][0] == 2 {
            if new_x2 < 0 || new_x2 >= 10 || new_y2 < 0 || new_y2 >= 10 ||
                players_board[new_x2 as usize][new_y2 as usize][0] == 3 {
                loop {
                    x = (x as i32 + x2) as usize;
                    y = (y as i32 + y2) as usize;
                    
                    if players_board[x][y][0] == 0 || players_board[x][y][0] == 1 {
                        return (x, y);
                    }
                }
            }
            return (new_x2 as usize, new_y2 as usize);
        }
    }
    return (10, 10);
}

fn go_opposite_direction(players_board: &mut [[[i32; 2]; 10]; 10], (x, y): (usize, usize), possibilities: [(i32, i32); 4]) -> (usize, usize) {
    for &(x2, y2) in &possibilities {
        let new_x: i32 = x as i32 + x2;
        let new_y: i32 = y as i32 + y2;
        if new_x < 0 || new_x >= 10 || new_y < 0 || new_y >= 10 {
            continue;
        }
        if players_board[new_x as usize][new_y as usize][0] != 3 {
            return (new_x as usize, new_y as usize);
        }
    }
    return (1, 10);
}

fn continue_hitting_ship(players_board: &mut [[[i32; 2]; 10]; 10], last_hit: (usize, usize)) -> (usize, usize) {
    let (x, y): (usize, usize) = last_hit;

    let mut possibilities: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut rng = rand::thread_rng();
    possibilities.shuffle(&mut rng);

    if go_same_direction(players_board, (x, y), possibilities) != (10, 10) {
        return go_same_direction(players_board, (x, y), possibilities);
    }
    else if go_opposite_direction(players_board, (x, y), possibilities) != (1, 10) {
        return go_opposite_direction(players_board, (x, y), possibilities);
    }
    return ((x as i32 + possibilities[0].0) as usize, (y as i32 + possibilities[0].1) as usize);
}

fn find_optimal_move(heatmap: [[i32; 10]; 10]) -> (usize, usize) {
    let mut max_value = 0;
    let mut moves: Vec<(usize, usize)> = Vec::new();
    let mut rng = rand::thread_rng();

    for x in 0..10 {
        for y in 0..10 {
            max_value = max(max_value, heatmap[x][y]);
        }
    }
    for x in 0..10 {
        for y in 0..10 {
            if heatmap[x][y] == max_value {
                moves.push((x, y));
            }
        }
    }
    let idx = rng.gen_range(0..moves.len());
    let chosen_move: (usize, usize) = moves[idx];
    
    return chosen_move;
}

fn did_last_sink(players_board: &mut [[[i32; 2]; 10]; 10], last_hit: (usize, usize), players_ships_lifes: &mut [i32; 6]) -> bool {
    if players_ships_lifes[players_board[last_hit.0][last_hit.1][1] as usize] == 0 {
        return true;
    }
    return false;
}

pub fn computers_turn(players_board: &mut [[[i32; 2]; 10]; 10], players_ships_lifes: &mut [i32; 6], last_hit: (usize, usize)) -> (usize, usize) {
    let chosen_move: (usize, usize);

    if last_hit == (10, 10) || did_last_sink(players_board, last_hit, players_ships_lifes) {
        let heatmap: [[i32; 10]; 10] = create_heatmap(players_board, players_ships_lifes);
        chosen_move = find_optimal_move(heatmap);
    } else {
        chosen_move = continue_hitting_ship(players_board, last_hit);
    }
    
    game::GameState::hit(players_board, chosen_move, players_ships_lifes);

    return chosen_move;
}

fn create_heatmap(players_board: &mut [[[i32; 2]; 10]; 10], players_ships_lifes: &mut [i32; 6]) -> [[i32; 10]; 10] {
    let mut heatmap: [[i32; 10]; 10] = [[0; 10]; 10];

    for y in 0..10 {
        for x in 0..10 {
            for (index, &len) in MAX_SHIPS_LIFES.iter().enumerate() {
                if players_ships_lifes[index] == 0 {
                    continue;
                }
                for k in 0..2 {
                    if can_place_ship(&players_board, y, x, len as usize, k == 1, &TRIED1, &TRIED2) {
                        for i in 0..len {
                            if k == 1 {
                                heatmap[y][x + i as usize] += 1;
                            } else {
                                heatmap[y + i as usize][x] += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    return heatmap;
}