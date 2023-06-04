use std::collections::VecDeque;
use std::io;

const DIRS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let mut coins = Vec::with_capacity(2);
    let board: Vec<Vec<_>> = input
        .skip(2)
        .enumerate()
        .map(|(r, row)| {
            row.char_indices()
                .map(|(c, ch)| match ch {
                    'o' => {
                        coins.push((r as i8, c as i8));
                        false
                    }
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    println!("{}", simulate(board, coins).unwrap_or(-1));
}

fn simulate(board: Vec<Vec<bool>>, coins: Vec<(i8, i8)>) -> Option<i8> {
    let (width, height) = (board[0].len() as i8, board.len() as i8);
    let is_in_board = |(r, c): (i8, i8)| 0 <= r && r < height && 0 <= c && c < width;

    let mut queue = VecDeque::from([([coins[0], coins[1]], 0)]);

    while let Some((coins, time)) = queue.pop_front() {
        if time >= 10 {
            return None;
        }

        let next_time = time + 1;

        for dir in DIRS {
            let [moved_coin_one, moved_coin_two] = coins.map(|coin| {
                let moved = ((coin.0 + dir.0), (coin.1 + dir.1));

                if is_in_board(moved) && board[moved.0 as usize][moved.1 as usize] {
                    coin
                } else {
                    moved
                }
            });

            match (is_in_board(moved_coin_one), is_in_board(moved_coin_two)) {
                (true, false) | (false, true) => return Some(next_time),
                (false, false) => continue,
                _ => (),
            }

            if [moved_coin_one, moved_coin_two] == coins {
                continue;
            }

            queue.push_back(([moved_coin_one, moved_coin_two], next_time));
        }
    }

    None
}
