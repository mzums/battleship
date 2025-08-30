use rand::Rng;
use crate::{MAX_SHIPS_LIFES, EMPTY1, EMPTY2};


pub fn can_place_ship(board: &[[[i32; 2]; 10]; 10], x: usize, y: usize, length: usize, horizontal: bool, permitted1: &[i32], permitted2: &[i32]) -> bool {
    let (dx, dy) = if horizontal { (0, 1) } else { (1, 0) };

    for i in 0..length {
        let nx = x + i * dx;
        let ny = y + i * dy;


        if nx >= 10 || ny >= 10 || !permitted1.contains(&board[nx][ny][0]) {
            return false;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let sx = nx as isize + dx;
                let sy = ny as isize + dy;
                if sx >= 0 && sx < 10 && sy >= 0 && sy < 10 && !permitted2.contains(&board[sx as usize][sy as usize][0])
                {
                    return false;
                }
            }
        }
    }
    true
}

fn place_ship(board: &mut [[[i32; 2]; 10]; 10], length: usize, id: i32, ships_lifes: &mut [i32; 6]) {
    ships_lifes[id as usize] = length as i32;
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(0..10);
        let y = rng.gen_range(0..10);
        let horizontal = rng.gen_bool(0.5);
        if can_place_ship(board, x, y, length, horizontal, &EMPTY1, &EMPTY2) {
            if horizontal {
                for i in 0..length {
                    board[x][y + i][1] = id;
                    board[x][y + i][0] = 1;
                }
            } else {
                for i in 0..length {
                    board[x + i][y][1] = id;
                    board[x + i][y][0] = 1;
                }
            }
            break;
        }
    }
}

pub fn place_ships(board: &mut [[[i32; 2]; 10]; 10], ships_lifes: &mut [i32; 6]) {
    for i in 1..6 {
        place_ship(board,MAX_SHIPS_LIFES[i] as usize, i as i32, ships_lifes);
    }
}
