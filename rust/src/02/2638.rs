use std::io;

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 100;

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

    let time = simulate(&mut map[..height], width);

    println!("{time}");
}

fn simulate(map: &mut [[bool; WIDTH_MAX]], width: usize) -> i32 {
    let height = map.len();
    let mut time = 0;

    let is_pass = |r: usize, c: usize, visited: &[[bool; WIDTH_MAX]], map: &[[bool; WIDTH_MAX]]| {
        visited[r][c] || !map[r][c]
    };

    loop {
        time += 1;
        melt_cheese(map, width);

        let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
        let mut count = 0;

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if is_pass(y, x, &visited, map) {
                    continue;
                }

                visited[y][x] = true;
                let mut stack = vec![(y, x)];

                while let Some((r, c)) = stack.pop() {
                    count += 1;
                    let adjacents = [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)];

                    for (adj_r, adj_c) in adjacents {
                        if is_pass(adj_r, adj_c, &visited, map) {
                            continue;
                        }

                        visited[adj_r][adj_c] = true;
                        stack.push((adj_r, adj_c));
                    }
                }
            }
        }

        if count == 0 {
            return time;
        }
    }
}

fn melt_cheese(map: &mut [[bool; WIDTH_MAX]], width: usize) {
    let height = map.len();
    let mut visited = [[None; WIDTH_MAX]; HEIGHT_MAX];
    let mut melted = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in map.iter().enumerate() {
        melted[r][..width].copy_from_slice(&row[..width]);
    }

    let x_full_range: [usize; WIDTH_MAX] = std::array::from_fn(|i| i);
    let x_edges = [0, width - 1];

    for y in 0..height {
        let x_range = if y == 0 || y == height - 1 {
            &x_full_range[..width]
        } else {
            &x_edges
        };

        for &x in x_range {
            if visited[y][x].is_some() {
                continue;
            }

            visited[y][x] = Some(0_u8);
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(height - 1), c),
                    (r, (c + 1).min(width - 1)),
                ];

                for (adj_r, adj_c) in adjacents {
                    if map[adj_r][adj_c] {
                        let count = visited[adj_r][adj_c].get_or_insert(0);
                        *count += 1;

                        if *count >= 2 {
                            melted[adj_r][adj_c] = false;
                        }

                        continue;
                    }

                    if visited[adj_r][adj_c].is_some() {
                        continue;
                    }

                    visited[adj_r][adj_c] = Some(0);
                    stack.push((adj_r, adj_c));
                }
            }
        }
    }

    for (r, row) in map.iter_mut().enumerate() {
        row[..width].copy_from_slice(&melted[r][..width]);
    }
}
