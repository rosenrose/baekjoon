use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    const SIZE: usize = 10;
    let mut board = [[false; 100]; 100];
    let mut count = 0;

    for _ in 0..input.next().unwrap() {
        let (x_gap, y_gap) = (input.next().unwrap(), input.next().unwrap());

        for y in y_gap..y_gap + SIZE {
            for x in x_gap..x_gap + SIZE {
                if board[y][x] {
                    continue;
                }

                board[y][x] = true;
                count += 1;
            }
        }
    }

    println!("{count}");
}
