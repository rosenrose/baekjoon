#[derive(Default, Copy, Clone, Debug)]
enum Dirs {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

use std::io;
use Dirs::*;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let board: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();

    let max_block = product(0, &mut [Default::default(); 5], &board);

    println!("{max_block}");
}

fn product(depth: usize, selected: &mut [Dirs; 5], board: &[Vec<i32>]) -> i32 {
    if depth == selected.len() {
        return simulate(selected, board.to_owned());
    }

    [Up, Right, Down, Left]
        .iter()
        .map(|&dir| {
            selected[depth] = dir;
            product(depth + 1, selected, board)
        })
        .max()
        .unwrap()
}

fn simulate(selected: &mut [Dirs; 5], mut board: Vec<Vec<i32>>) -> i32 {
    let n = board.len();
    let mut merged = vec![vec![false; n]; n];

    for dir in selected {
        match dir {
            Up => {
                for r in 1..n {
                    for c in 0..n {
                        if board[r][c] == 0 {
                            continue;
                        }

                        let Some(up) = (0..r).rfind(|&r| board[r][c] != 0) else {
                            (board[0][c], board[r][c]) = (board[r][c], board[0][c]);
                            continue;
                        };

                        if board[up][c] == board[r][c] && !merged[up][c] {
                            board[up][c] <<= 1;
                            board[r][c] = 0;
                            merged[up][c] = true;
                        } else {
                            (board[up + 1][c], board[r][c]) = (board[r][c], board[up + 1][c]);
                        }
                    }
                }
            }
            Down => {
                for r in (0..n - 1).rev() {
                    for c in 0..n {
                        if board[r][c] == 0 {
                            continue;
                        }

                        let Some(down) = (r + 1..n).find(|&r| board[r][c] != 0) else {
                            (board[n - 1][c], board[r][c]) = (board[r][c], board[n - 1][c]);
                            continue;
                        };

                        if board[down][c] == board[r][c] && !merged[down][c] {
                            board[down][c] <<= 1;
                            board[r][c] = 0;
                            merged[down][c] = true;
                        } else {
                            (board[down - 1][c], board[r][c]) = (board[r][c], board[down - 1][c]);
                        }
                    }
                }
            }
            Right => {
                for r in 0..n {
                    for c in (0..n - 1).rev() {
                        if board[r][c] == 0 {
                            continue;
                        }

                        let Some(right) = (c + 1..n).find(|&c| board[r][c] != 0) else {
                            board[r].swap(n - 1, c);
                            continue;
                        };

                        if board[r][right] == board[r][c] && !merged[r][right] {
                            board[r][right] <<= 1;
                            board[r][c] = 0;
                            merged[r][right] = true;
                        } else {
                            board[r].swap(right - 1, c);
                        }
                    }
                }
            }
            Left => {
                for r in 0..n {
                    for c in 1..n {
                        if board[r][c] == 0 {
                            continue;
                        }

                        let Some(left) = (0..c).rfind(|&c| board[r][c] != 0) else {
                            board[r].swap(0, c);
                            continue;
                        };

                        if board[r][left] == board[r][c] && !merged[r][left] {
                            board[r][left] <<= 1;
                            board[r][c] = 0;
                            merged[r][left] = true;
                        } else {
                            board[r].swap(left + 1, c);
                        }
                    }
                }
            }
        }

        for r in 0..n {
            for c in 0..n {
                merged[r][c] = false;
            }
        }
    }

    *board.iter().flatten().max().unwrap()
}
