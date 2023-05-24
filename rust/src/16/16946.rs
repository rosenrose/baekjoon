use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let (height, width) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );
    let map: Vec<_> = input.map(str::as_bytes).collect();

    let mut visited = vec![vec![false; width as usize]; height as usize];
    let mut counts = vec![vec![1; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let start = (y as usize, x as usize);

            if visited[start.0][start.1] || map[start.0][start.1] == b'1' {
                continue;
            }

            visited[start.0][start.1] = true;
            let mut count = 0;
            let mut walls = Vec::new();
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                count += 1;
                counts[r as usize][c as usize] = 0;

                let adjacents = [
                    ((r - 1).max(0), c),
                    (r, (c - 1).max(0)),
                    ((r + 1).min(height - 1), c),
                    (r, (c + 1).min(width - 1)),
                ];

                for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
                    let adj = (adj_r as usize, adj_c as usize);

                    if visited[adj.0][adj.1] {
                        continue;
                    }

                    visited[adj.0][adj.1] = true;

                    if map[adj.0][adj.1] == b'1' {
                        walls.push((adj_r, adj_c));
                        continue;
                    }

                    stack.push((adj_r, adj_c));
                }
            }

            for (r, c) in walls {
                let wall = (r as usize, c as usize);

                counts[wall.0][wall.1] = ((counts[wall.0][wall.1] as i32 + count) % 10) as u8;
                visited[wall.0][wall.1] = false;
            }
        }
    }

    for row in counts {
        for count in row {
            write!(output, "{count}").unwrap();
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i16 {
    buf.parse().unwrap()
}
