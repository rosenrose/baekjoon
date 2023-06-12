use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let [height, width, k] = [(); 3].map(|_| input());
    let mut map = vec![vec![false; width]; height];

    for (r, c) in (0..k).map(|_| (input() - 1, input() - 1)) {
        map[r][c] = true;
    }

    let mut max_size = 0;
    let mut visited = vec![vec![false; width]; height];

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

            max_size = count.max(max_size);
        }
    }

    println!("{max_size}");
}
