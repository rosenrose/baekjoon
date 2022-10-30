fn main() {
    let mut buf = String::new();

    let board = parse_str_vec_lines(&mut buf, 8);
    let mut count = 0;

    for (i, row) in board.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            match (i % 2, j % 2, c) {
                (0, 0, 'F') | (1, 1, 'F') => count += 1,
                _ => (),
            };
        }
    }

    println!("{count}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: i32) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
