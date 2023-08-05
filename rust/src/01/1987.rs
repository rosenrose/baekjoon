use std::io;

const OFFSET: u8 = b'A';

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let board: Vec<_> = input.skip(2).map(str::as_bytes).collect();
    let mut visited = [false; 26];
    visited[(board[0][0] - OFFSET) as usize] = true;

    let max_depth = board_horse(1, (0, 0), &mut visited, &board);

    println!("{max_depth}");
}

fn board_horse(depth: i32, (r, c): (usize, usize), visited: &mut [bool], board: &[&[u8]]) -> i32 {
    let adjacents = [
        (r.saturating_sub(1), c),
        (r, c.saturating_sub(1)),
        ((r + 1).min(board.len() - 1), c),
        (r, (c + 1).min(board[0].len() - 1)),
    ];

    adjacents
        .iter()
        .filter_map(|&(adj_r, adj_c)| {
            let idx = (board[adj_r][adj_c] - OFFSET) as usize;

            (!visited[idx]).then(|| {
                visited[idx] = true;

                let result = board_horse(depth + 1, (adj_r, adj_c), visited, board);
                visited[idx] = false;

                result
            })
        })
        .max()
        .unwrap_or(depth)
}
