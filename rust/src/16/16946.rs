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
    let mut map: Vec<Vec<_>> = input
        .map(|row| row.as_bytes().iter().map(|ch| (ch - b'0') as i32).collect())
        .collect();

    let mut visited = vec![vec![false; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let start = (y as usize, x as usize);

            if visited[start.0][start.1] || map[start.0][start.1] != 0 {
                continue;
            }

            visited[start.0][start.1] = true;
            let mut count = 0;
            let mut walls = Vec::new();
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                count += 1;

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

                    if map[adj.0][adj.1] != 0 {
                        walls.push((adj_r, adj_c));
                        continue;
                    }

                    stack.push((adj_r, adj_c));
                }
            }

            for (r, c) in walls {
                let wall = (r as usize, c as usize);

                map[wall.0][wall.1] += count;
                visited[wall.0][wall.1] = false;
            }
        }
    }

    for row in map {
        for count in row {
            write!(output, "{}", count % 10).unwrap();
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i16 {
    buf.parse().unwrap()
}
