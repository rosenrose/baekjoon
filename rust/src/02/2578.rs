use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let mut board = [[(0, false); 5]; 5];
    let mut count = 0;

    for i in 0..5 {
        for j in 0..5 {
            board[i][j].0 = input();
        }
    }

    for order in 1..=25 {
        count += draw_bingo(&mut board, input());

        if count >= 3 {
            println!("{order}");
            return;
        }
    }
}

fn draw_bingo(board: &mut [[(i32, bool); 5]; 5], num: i32) -> i32 {
    let mut count = 0;

    let (i, j) = 'outer: {
        for i in 0..5 {
            for j in 0..5 {
                if board[i][j].0 == num {
                    board[i][j].1 = true;

                    break 'outer (i, j);
                }
            }
        }
        unreachable!()
    };

    if (0..5).all(|k| board[i][k].1) {
        count += 1;
    }
    if (0..5).all(|k| board[k][j].1) {
        count += 1;
    }
    if i == j && (0..5).all(|k| board[k][k].1) {
        count += 1;
    }
    if i + j == 4 && (0..5).all(|k| board[k][4 - k].1) {
        count += 1;
    }

    count
}
