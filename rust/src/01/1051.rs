use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let (n, m) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );
    let matrix: Vec<_> = input.map(|row| row.as_bytes()).collect();
    let max_size = n.min(m);

    for size in (1..=max_size).rev() {
        for y in 0..=n - size {
            for x in 0..=m - size {
                let top = matrix[y];
                let bottom = matrix[y + size - 1];

                let (top_left, top_right) = (top[x], top[x + size - 1]);
                let (bottom_left, bottom_right) = (bottom[x], bottom[x + size - 1]);

                if top_left == top_right && top_right == bottom_left && bottom_left == bottom_right
                {
                    println!("{}", size * size);
                    return;
                }
            }
        }
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
