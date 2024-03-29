use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [height, width] = [(); 2].map(|_| input.next().unwrap());
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| input.by_ref().take(width).map(|num| num == 1).collect())
        .collect();

    let (time, count) = simulate(map);

    println!("{time}\n{count}");
}

fn simulate(mut map: Vec<Vec<bool>>) -> (i32, i32) {
    let mut time = 0;
    let mut prev_count = get_count(&map);

    loop {
        time += 1;
        melt_cheese(&mut map);

        let count = get_count(&map);

        if count == 0 {
            return (time, prev_count);
        }

        prev_count = count;
    }
}

fn get_count(map: &[Vec<bool>]) -> i32 {
    let (width, height) = (map[0].len(), map.len());
    let mut visited = vec![vec![false; width]; height];
    let mut count = 0;

    let is_pass = |r: usize, c: usize, visited: &[Vec<bool>]| visited[r][c] || !map[r][c];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if is_pass(y, x, &visited) {
                continue;
            }

            visited[y][x] = true;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                count += 1;
                let adjacents = [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)];

                for (adj_r, adj_c) in adjacents {
                    if is_pass(adj_r, adj_c, &visited) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }
        }
    }

    count
}

fn melt_cheese(map: &mut Vec<Vec<bool>>) {
    let (width, height) = (map[0].len(), map.len());
    let mut visited = vec![vec![false; width]; height];
    let mut melted = map.clone();

    let x_full_range: Vec<_> = (0..width).collect();
    let x_edges = [0, width - 1];

    for y in 0..height {
        let x_range = if y == 0 || y == height - 1 {
            &x_full_range[..]
        } else {
            &x_edges
        };

        for &x in x_range {
            if visited[y][x] {
                continue;
            }

            visited[y][x] = true;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(height - 1), c),
                    (r, (c + 1).min(width - 1)),
                ];

                for (adj_r, adj_c) in adjacents {
                    if visited[adj_r][adj_c] {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;

                    if map[adj_r][adj_c] {
                        melted[adj_r][adj_c] = false;
                        continue;
                    }

                    stack.push((adj_r, adj_c));
                }
            }
        }
    }

    *map = melted;
}
