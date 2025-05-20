use std::io;

const WIDTH_MAX: usize = 500;
const HEIGHT_MAX: usize = 500;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width] = [(); 2].map(|_| input.next().unwrap());
    let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for r in 0..height {
        for (c, num) in input.by_ref().take(width).enumerate() {
            map[r][c] = num == 1;
        }
    }

    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
    let mut areas = [0; HEIGHT_MAX * WIDTH_MAX / 2];
    let mut areas_len = 0;

    let is_pass = |r: usize, c: usize, visited: &[[bool; WIDTH_MAX]]| visited[r][c] || !map[r][c];

    for y in 0..height {
        for x in 0..width {
            if is_pass(y, x, &visited) {
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
                    if is_pass(adj_r, adj_c, &visited) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            areas[areas_len] = count;
            areas_len += 1;
        }
    }

    println!(
        "{}\n{}",
        areas_len,
        areas[..areas_len].iter().max().unwrap_or(&0)
    );
}
