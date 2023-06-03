use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let (height, width) = (input.next().unwrap(), input.next().unwrap());
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| input.by_ref().take(width).map(|num| num == 1).collect())
        .collect();

    let mut visited = vec![vec![false; width]; height];
    let mut areas = Vec::new();

    let is_pass = |r: usize, c: usize, visited: &[Vec<bool>]| visited[r][c] || !map[r][c];

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

            areas.push(count);
        }
    }

    println!("{}\n{}", areas.len(), areas.iter().max().unwrap_or(&0));
}
