#[derive(Default, Copy, Clone, Debug)]
enum Dirs {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

use std::collections::HashSet;
use std::io;
use Dirs::{Down, Left, Right, Up};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let board: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();

    let max_block = product(0, &mut [Default::default(); 5], &board, &n);

    println!("{max_block}");
}

fn product(depth: usize, selected: &mut [Dirs; 5], board: &Vec<Vec<i32>>, size: &usize) -> i32 {
    if depth == selected.len() {
        return simulate(selected, board.clone(), size);
    }

    [Up, Right, Down, Left]
        .iter()
        .map(|&dir| {
            selected[depth] = dir;
            product(depth + 1, selected, board, size)
        })
        .max()
        .unwrap()
}

fn simulate(selected: &mut [Dirs; 5], mut board: Vec<Vec<i32>>, &size: &usize) -> i32 {
    let mut merged = HashSet::new();

    for dir in selected {
        match dir {
            Up => {
                for r in 1..size {
                    for c in 0..size {
                        if board[r][c] == 0 {
                            continue;
                        }
                        if board[0][c] == 0 {
                            (board[0][c], board[r][c]) = (board[r][c], board[0][c]);
                            continue;
                        }

                        let up = (0..r).rfind(|&r| board[r][c] != 0).unwrap();

                        if board[up][c] == board[r][c] && !merged.contains(&(up, c)) {
                            board[up][c] <<= 1;
                            board[r][c] = 0;
                            merged.insert((up, c));
                        } else {
                            (board[up + 1][c], board[r][c]) = (board[r][c], board[up + 1][c]);
                        }
                    }
                }
            }
            Down => {
                for r in (0..size - 1).rev() {
                    for c in 0..size {
                        if board[r][c] == 0 {
                            continue;
                        }
                        if board[size - 1][c] == 0 {
                            (board[size - 1][c], board[r][c]) = (board[r][c], board[size - 1][c]);
                            continue;
                        }

                        let down = (r + 1..size).find(|&r| board[r][c] != 0).unwrap();

                        if board[down][c] == board[r][c] && !merged.contains(&(down, c)) {
                            board[down][c] <<= 1;
                            board[r][c] = 0;
                            merged.insert((down, c));
                        } else {
                            (board[down - 1][c], board[r][c]) = (board[r][c], board[down - 1][c]);
                        }
                    }
                }
            }
            Right => {
                for r in 0..size {
                    for c in (0..size - 1).rev() {
                        if board[r][c] == 0 {
                            continue;
                        }
                        if board[r][size - 1] == 0 {
                            (board[r][size - 1], board[r][c]) = (board[r][c], board[r][size - 1]);
                            continue;
                        }

                        let right = (c + 1..size).find(|&c| board[r][c] != 0).unwrap();

                        if board[r][right] == board[r][c] && !merged.contains(&(r, right)) {
                            board[r][right] <<= 1;
                            board[r][c] = 0;
                            merged.insert((r, right));
                        } else {
                            (board[r][right - 1], board[r][c]) = (board[r][c], board[r][right - 1]);
                        }
                    }
                }
            }
            Left => {
                for r in 0..size {
                    for c in 1..size {
                        if board[r][c] == 0 {
                            continue;
                        }
                        if board[r][0] == 0 {
                            (board[r][0], board[r][c]) = (board[r][c], board[r][0]);
                            continue;
                        }

                        let left = (0..c).rfind(|&c| board[r][c] != 0).unwrap();

                        if board[r][left] == board[r][c] && !merged.contains(&(r, left)) {
                            board[r][left] <<= 1;
                            board[r][c] = 0;
                            merged.insert((r, left));
                        } else {
                            (board[r][left + 1], board[r][c]) = (board[r][c], board[r][left + 1]);
                        }
                    }
                }
            }
        }

        merged.clear();
    }

    *board
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
}
