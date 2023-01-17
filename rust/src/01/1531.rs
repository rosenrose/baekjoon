use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut board = [[0; 101]; 101];
    let mut count = 0;

    for (x1, y1, x2, y2) in (0..n).map(|_| (input(), input(), input(), input())) {
        for i in y1..=y2 {
            for j in x1..=x2 {
                board[i][j] += 1;

                if board[i][j] == m + 1 {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}
