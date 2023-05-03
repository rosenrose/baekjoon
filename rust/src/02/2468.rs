use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let (mut min_height, mut max_height) = (100, 1);
    let area: Vec<Vec<_>> = (0..n)
        .map(|_| {
            input
                .by_ref()
                .take(n)
                .map(|height| {
                    min_height = height.min(min_height);
                    max_height = height.max(max_height);
                    height
                })
                .collect()
        })
        .collect();

    let max_safe_area = (min_height..=max_height - 1)
        .map(|height| get_safe_area(height, &area))
        .max()
        .unwrap_or(1);

    println!("{max_safe_area}");
}

fn get_safe_area(height: i32, area: &[Vec<i32>]) -> i32 {
    let n = area.len();
    let mut count = 0;
    let mut visited = vec![vec![false; n]; n];

    let is_pass = |r: usize, c: usize, visited: &[Vec<bool>]| area[r][c] <= height || visited[r][c];

    for y in 0..n {
        for x in 0..n {
            if is_pass(y, x, &visited) {
                continue;
            }

            visited[y][x] = true;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(n - 1), c),
                    (r, (c + 1).min(n - 1)),
                ];

                for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
                    if is_pass(adj_r, adj_c, &visited) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            count += 1;
        }
    }

    count
}
