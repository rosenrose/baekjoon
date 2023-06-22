use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let map: Vec<_> = input.skip(2).map(str::as_bytes).collect();

    println!("{}", if dfs(&map) { "Yes" } else { "No" });
}

fn dfs(map: &[&[u8]]) -> bool {
    let (width, height) = (map[0].len(), map.len());
    let mut visited = vec![vec![None; width]; height];

    for y in 0..height {
        for x in 0..width {
            if visited[y][x].is_some() {
                continue;
            }

            visited[y][x] = Some((y, x));
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                let parent = visited[r][c].unwrap();
                let color = map[r][c];
                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(height - 1), c),
                    (r, (c + 1).min(width - 1)),
                ]
                .into_iter()
                .filter(|&(adj_r, adj_c)| {
                    (adj_r, adj_c) != (r, c)
                        && (adj_r, adj_c) != parent
                        && map[adj_r][adj_c] == color
                });

                for (adj_r, adj_c) in adjacents {
                    if visited[adj_r][adj_c].is_some() {
                        return true;
                    }

                    visited[adj_r][adj_c] = Some((r, c));
                    stack.push((adj_r, adj_c));
                }
            }
        }
    }

    false
}
