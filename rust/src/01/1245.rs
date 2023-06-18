use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width] = [(); 2].map(|_| input.next().unwrap());
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| input.by_ref().take(width).collect())
        .collect();

    let mut count = 0;
    let mut visited = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            if visited[y][x] {
                continue;
            }

            visited[y][x] = true;
            let mut is_peak = true;
            let mut stack = vec![(y, x)];

            while let Some((row, col)) = stack.pop() {
                let altitude = map[row][col];
                let (up, down, left, right) = (
                    row.saturating_sub(1),
                    (row + 1).min(height - 1),
                    col.saturating_sub(1),
                    (col + 1).min(width - 1),
                );
                let adjacents = [
                    (up, left),
                    (up, col),
                    (up, right),
                    (row, left),
                    (row, right),
                    (down, left),
                    (down, col),
                    (down, right),
                ];

                for (adj_r, adj_c) in adjacents {
                    if map[adj_r][adj_c] > altitude {
                        is_peak = false;
                    }

                    if visited[adj_r][adj_c] || map[adj_r][adj_c] != altitude {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            if is_peak {
                count += 1;
            }
        }
    }

    println!("{count}");
}
