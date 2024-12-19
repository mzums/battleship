use rand::Rng;

fn print_board(players_board: &[[i32; 10]; 10], computers_board: &[[i32; 10]; 10]) {
    println!("   A B C D E F G H I J         A B C D E F G H I J");
    for i in 0..10 {
        print_board_line(players_board, i);
        print!("     ");
        print_board_line(computers_board, i);
        println!();
    }
    println!("      Your board                Opponent's board");
}

fn print_board_line(board: &[[i32; 10]; 10], i: usize) {
    print!("{} ", i);
    for j in 0..10 {
        print!("|");
        if board[i][j] == 0 {
            print!(" ");
        } else {
            print!("X");
        }
    }
    print!("|");
}

fn can_place_ship(board: &[[i32; 10]; 10], x: usize, y: usize, length: usize, horizontal: bool) -> bool {
    let (dx, dy) = if horizontal { (0, 1) } else { (1, 0) };

    for i in 0..length {
        let nx = x + i * dx;
        let ny = y + i * dy;

        if nx >= 10 || ny >= 10 || board[nx][ny] != 0 {
            return false;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                let sx = nx as isize + dx;
                let sy = ny as isize + dy;
                if sx >= 0 && sx < 10 && sy >= 0 && sy < 10 && board[sx as usize][sy as usize] != 0 {
                    return false;
                }
            }
        }
    }
    true
}


fn place_ship(board: &mut [[i32; 10]; 10], length: usize) {
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(0..10);
        let y = rng.gen_range(0..10);
        let horizontal = rng.gen_bool(0.5);
        if can_place_ship(board, x, y, length, horizontal) {
            if horizontal {
                for i in 0..length {
                    board[x][y + i] = 1;
                }
            } else {
                for i in 0..length {
                    board[x + i][y] = 1;
                }
            }
            break;
        }
    }
}

fn place_ships(board: &mut [[i32; 10]; 10]) {
    place_ship(board, 5);
    place_ship(board, 4);
    place_ship(board, 3);
    place_ship(board, 3); 
    place_ship(board, 2);
}

fn main() {
    println!("Hello, world!");
    let mut players_board: [[i32; 10]; 10] = [[0; 10]; 10];
    let mut computers_board: [[i32; 10]; 10] = [[0; 10]; 10];

    place_ships(&mut players_board);
    place_ships(&mut computers_board);
    print_board(&players_board, &computers_board);
}
