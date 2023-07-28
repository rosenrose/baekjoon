use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [width, height] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let map: Vec<_> = input.map(str::as_bytes).collect();

    let mut visited = vec![vec![false; width]; height];
    let (mut white, mut blue) = (0, 0);

    for y in 0..height {
        for x in 0..width {
            if visited[y][x] {
                continue;
            }

            visited[y][x] = true;
            let mut count = 0;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                count += 1;

                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(height - 1), c),
                    (r, (c + 1).min(width - 1)),
                ];

                for (adj_r, adj_c) in adjacents {
                    if visited[adj_r][adj_c] || map[r][c] != map[adj_r][adj_c] {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            let power = count * count;

            if map[y][x] == b'W' {
                white += power;
            } else {
                blue += power;
            }
        }
    }

    println!("{white} {blue}");
}
