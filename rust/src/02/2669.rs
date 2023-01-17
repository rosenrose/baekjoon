use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let mut board = [[false; 100]; 100];
    let mut count = 0;

    for (x1, y1, x2, y2) in (0..4).map(|_| (input(), input(), input(), input())) {
        for i in y1..y2 {
            for j in x1..x2 {
                if board[i][j] {
                    continue;
                }

                board[i][j] = true;
                count += 1;
            }
        }
    }

    println!("{count}");
}
