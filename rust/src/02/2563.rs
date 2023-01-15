use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    const SIZE: usize = 10;
    let mut board = [[false; 100]; 100];
    let mut count = 0;

    for (x_gap, y_gap) in (0..input()).map(|_| (input(), input())) {
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
