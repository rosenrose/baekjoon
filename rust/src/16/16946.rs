use std::fmt::Write;
use std::io;

const WIDTH_MAX: usize = 1000;
const HEIGHT_MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.map(str::as_bytes).enumerate() {
        for (c, &num) in row.iter().enumerate() {
            map[r][c] = num;
        }
    }

    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
    let mut counts = [[1; WIDTH_MAX]; HEIGHT_MAX];

    for y in 0..height as i16 {
        for x in 0..width as i16 {
            let start = (y as usize, x as usize);

            if visited[start.0][start.1] || map[start.0][start.1] == b'1' {
                continue;
            }

            visited[start.0][start.1] = true;
            let mut count = 0;
            let mut walls = Vec::with_capacity(WIDTH_MAX * HEIGHT_MAX / 2);
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                count += 1;
                counts[r as usize][c as usize] = 0;

                let adjacents = [
                    ((r - 1).max(0), c),
                    (r, (c - 1).max(0)),
                    ((r + 1).min(height as i16 - 1), c),
                    (r, (c + 1).min(width as i16 - 1)),
                ];

                for (adj_r, adj_c) in adjacents {
                    let adj = (adj_r as usize, adj_c as usize);

                    if visited[adj.0][adj.1] {
                        continue;
                    }

                    visited[adj.0][adj.1] = true;

                    if map[adj.0][adj.1] == b'1' {
                        walls.push((adj_r as i16, adj_c as i16));
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

    for row in &counts[..height] {
        for count in &row[..width] {
            write!(output, "{count}").unwrap();
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}
