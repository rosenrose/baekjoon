fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [n, m] = parse_int_vec(&buf)[..] {
        let matrix = parse_matrix(&mut buf, n);
        let max_size = n.min(m);

        for size in (1..=max_size).rev() {
            for y in 0..=n - size {
                for x in 0..=m - size {
                    let top_bytes = matrix[y].as_bytes();
                    let bottom_bytes = matrix[y + size - 1].as_bytes();

                    let (top_left, top_right) = (top_bytes[x], top_bytes[x + size - 1]);
                    let (bottom_left, bottom_right) = (bottom_bytes[x], bottom_bytes[x + size - 1]);

                    if top_left == top_right
                        && top_right == bottom_left
                        && bottom_left == bottom_right
                    {
                        println!("{}", size * size);
                        return;
                    }
                }
            }
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_matrix(buf: &mut String, rows: usize) -> Vec<String> {
    (0..rows)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
