fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    const SIZE: usize = 10;
    let mut board = [[false; 100]; 100];
    let mut count = 0;

    for _ in 0..n {
        read_line(&mut buf);

        let gap = parse_int_vec(&buf);
        let (x_gap, y_gap) = (gap[0], gap[1]);

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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
