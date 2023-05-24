use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (height, width) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );
    let mut map: Vec<Vec<_>> = input.map(|row| row.as_bytes().to_owned()).collect();

    for x in 0..width {
        if map[0][x] == b'1' {
            continue;
        }

        map[0][x] = b'1';
        let mut stack = vec![(0, x)];

        while let Some((r, c)) = stack.pop() {
            if r == height - 1 {
                println!("YES");
                return;
            }

            let adjacents = [
                (r.saturating_sub(1), c),
                (r, c.saturating_sub(1)),
                ((r + 1).min(height - 1), c),
                (r, (c + 1).min(width - 1)),
            ];

            for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
                if map[adj_r][adj_c] == b'1' {
                    continue;
                }

                map[adj_r][adj_c] = b'1';
                stack.push((adj_r, adj_c));
            }
        }
    }

    println!("NO");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
