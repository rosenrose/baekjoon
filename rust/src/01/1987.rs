use std::io;

const OFFSET: usize = 'A' as usize;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (_r, _c) = (input.next(), input.next());
    let board: Vec<_> = input.map(str::as_bytes).collect();

    let mut is_passed = [false; 26];
    is_passed[board[0][0] as usize - OFFSET] = true;

    let max_depth = board_horse(1, (0, 0), &mut is_passed, &board);
    println!("{max_depth}");
}

fn board_horse(
    depth: i32,
    (r, c): (usize, usize),
    is_passed: &mut [bool; 26],
    board: &[&[u8]],
) -> i32 {
    let adjacents = [
        (r.saturating_sub(1), c),
        (r, c.saturating_sub(1)),
        ((r + 1).min(board.len() - 1), c),
        (r, (c + 1).min(board[0].len() - 1)),
    ];

    adjacents
        .iter()
        .filter_map(|&(adj_r, adj_c)| {
            let idx = board[adj_r][adj_c] as usize - OFFSET;

            ((adj_r, adj_c) != (r, c) && !is_passed[idx]).then(|| {
                is_passed[idx] = true;

                let result = board_horse(depth + 1, (adj_r, adj_c), is_passed, board);
                is_passed[idx] = false;

                result
            })
        })
        .max()
        .unwrap_or(depth)
}
