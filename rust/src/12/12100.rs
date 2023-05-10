#[derive(Default, Copy, Clone, Debug)]
enum Dirs {
    #[default]
    Up = 0,
    Down,
    Left,
    Right,
}

use std::io;
use Dirs::*;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let board: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();
    let rows_cols = [
        ((1..n).collect(), (0..n).collect()),
        ((0..n - 1).rev().collect(), (0..n).collect()),
        ((0..n).collect(), (1..n).collect()),
        ((0..n).collect(), (0..n - 1).rev().collect()),
    ];

    let max_block = product(0, &mut [Default::default(); 5], &board, &rows_cols);

    println!("{max_block}");
}

fn product(
    depth: usize,
    selected: &mut [Dirs; 5],
    board: &[Vec<i32>],
    rows_cols: &[(Vec<usize>, Vec<usize>)],
) -> i32 {
    if depth == selected.len() {
        return simulate(selected, board.to_owned(), rows_cols);
    }

    [Up, Right, Down, Left]
        .iter()
        .map(|&dir| {
            selected[depth] = dir;
            product(depth + 1, selected, board, rows_cols)
        })
        .max()
        .unwrap()
}

fn simulate(
    selected: &mut [Dirs; 5],
    mut board: Vec<Vec<i32>>,
    rows_cols: &[(Vec<usize>, Vec<usize>)],
) -> i32 {
    let n = board.len();
    let mut merged = vec![vec![false; n]; n];

    for &dir in selected.iter() {
        let (rows, cols) = &rows_cols[dir as usize];

        for &r in rows.iter() {
            for &c in cols.iter() {
                if board[r][c] == 0 {
                    continue;
                }

                let Some(target) = (match dir {
                    Up => (0..r).rfind(|&target_r| board[target_r][c] != 0),
                    Down => (r + 1..n).find(|&target_r| board[target_r][c] != 0),
                    Left => (0..c).rfind(|&target_c| board[r][target_c] != 0),
                    Right => (c + 1..n).find(|&target_c| board[r][target_c] != 0),
                }) else {
                    match dir {
                        Up => (board[0][c], board[r][c]) = (board[r][c], board[0][c]),
                        Down => (board[n - 1][c], board[r][c]) = (board[r][c], board[n - 1][c]),
                        Left => board[r].swap(0, c),
                        Right => board[r].swap(n - 1, c),
                    }

                    continue;
                };

                let (target_r, target_c) = match dir {
                    Up | Down => (target, c),
                    Left | Right => (r, target),
                };

                if board[target_r][target_c] == board[r][c] && !merged[target_r][target_c] {
                    board[target_r][target_c] <<= 1;
                    board[r][c] = 0;
                    merged[target_r][target_c] = true;

                    continue;
                }

                match dir {
                    Up => (board[target + 1][c], board[r][c]) = (board[r][c], board[target + 1][c]),
                    #[rustfmt::skip]
                    Down => (board[target - 1][c], board[r][c]) = (board[r][c], board[target - 1][c]),
                    Left => board[r].swap(target + 1, c),
                    Right => board[r].swap(target - 1, c),
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
