use std::io;
use rand::{thread_rng, Rng};

const SIZE: usize = 4;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    board: [[u32; SIZE]; SIZE],
    score: u32,
}

impl Game {
    fn new() -> Game {
        let mut game = Game {
            board: [[0; SIZE]; SIZE],
            score: 0,
        };
        game.add_random_tile();
        game.add_random_tile();
        game
    }

    fn add_random_tile(&mut self) {
        let mut rng = thread_rng();
        let mut empty_cells = vec![];

        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.board[row][col] == 0 {
                    empty_cells.push((row, col));
                }
            }
        }

        if empty_cells.is_empty() {
            return;
        }

        let (row, col) = empty_cells[rng.gen_range(0..empty_cells.len())];
        self.board[row][col] = if rng.gen::<f64>() < 0.9 { 2 } else { 4 };
    }

    fn is_game_over(&self) -> bool {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.board[row][col] == 0 {
                    return false;
                }

                if row > 0 && self.board[row][col] == self.board[row - 1][col] {
                    return false;
                }

                if row < SIZE - 1 && self.board[row][col] == self.board[row + 1][col] {
                    return false;
                }

                if col > 0 && self.board[row][col] == self.board[row][col - 1] {
                    return false;
                }

                if col < SIZE - 1 && self.board[row][col] == self.board[row][col + 1] {
                    return false;
                }
            }
        }

        true
    }

    fn print_board(&self) {
        println!("Score: {}", self.score);
        for row in &self.board {
            for cell in row {
                print!("{:<5}", cell);
            }
            println!();
        }
    }

    fn apply_move(&mut self, dir: Direction) {
        let mut merged = [[false; SIZE]; SIZE];
        let mut moved = false;

        for _ in 0..SIZE {
            for row in 0..SIZE {
                for col in 0..SIZE {
                    let (next_row, next_col) = match dir {
                        Direction::Up => (row as isize - 1, col as isize),
                        Direction::Down => (row as isize + 1, col as isize),
                        Direction::Left => (row as isize, col as isize - 1),
                        Direction::Right => (row as isize, col as isize + 1),
                    };

                    if next_row >= 0 && next_row < SIZE as isize
                        && next_col >= 0 && next_col < SIZE as isize
                        && self.board[next_row as usize][next_col as usize] == self.board[row][col]
                        && !merged[next_row as usize][next_col as usize]
                        && !merged[row][col] {
                        self.board[row][col] *= 2;
                        self.board[next_row as usize][next_col as usize] = 0;
                        self.score += self.board[row][col];
                        merged[row][col] = true;
                        moved = true;
                    }

                    if self.board[row][col] == 0 {
                        continue;
                    }

                    let mut new_row = row as isize;
                    let mut new_col = col as isize;

                    while new_row > 0 && new_col > 0 {
                        let (prev_row, prev_col) = match dir {
                            Direction::Up => (new_row - 1, new_col),
                            Direction::Down => (new_row + 1, new_col),
                            Direction::Left => (new_row, new_col - 1),
                            Direction::Right => (new_row, new_col + 1),
                        };

                        if self.board[prev_row as usize][prev_col as usize] == 0 {
                            self.board[prev_row as usize][prev_col as usize] = self.board[new_row as usize][new_col as usize];
                            self.board[new_row as usize][new_col as usize] = 0;
                            new_row = prev_row;
                            new_col = prev_col;
                            moved = true;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        if moved {
            self.add_random_tile();
        }
    }
}

fn main() {
    let mut game = Game::new();

    while !game.is_game_over() {
        game.print_board();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "w" => game.apply_move(Direction::Up),
            "s" => game.apply_move(Direction::Down),
            "a" => game.apply_move(Direction::Left),
            "d" => game.apply_move(Direction::Right),
            _ => continue,
        }
    }

    println!("Game Over!");
}