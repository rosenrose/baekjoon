use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let map: Vec<_> = input.skip(2).map(str::as_bytes).collect();

    println!("{}", if is_cycle(&map) { "Yes" } else { "No" });
}

fn is_cycle(map: &[&[u8]]) -> bool {
    let (width, height) = (map[0].len(), map.len());
    let mut visited = vec![vec![None; width]; height];

    for r in 0..height {
        for c in 0..width {
            if visited[r][c].is_some() {
                continue;
            }

            visited[r][c] = Some((r, c));

            if dfs((r, c), map, &mut visited) {
                return true;
            }
        }
    }

    false
}

fn dfs(
    (r, c): (usize, usize),
    map: &[&[u8]],
    visited: &mut Vec<Vec<Option<(usize, usize)>>>,
) -> bool {
    let (width, height) = (map[0].len(), map.len());
    let parent = visited[r][c].unwrap();
    let color = map[r][c];

    let mut adjacents = [
        (r.saturating_sub(1), c),
        (r, c.saturating_sub(1)),
        ((r + 1).min(height - 1), c),
        (r, (c + 1).min(width - 1)),
    ]
    .into_iter()
    .filter(|&(adj_r, adj_c)| {
        (adj_r, adj_c) != (r, c) && (adj_r, adj_c) != parent && map[adj_r][adj_c] == color
    });

    adjacents.any(|(adj_r, adj_c)| {
        if visited[adj_r][adj_c].is_some() {
            return true;
        }

        visited[adj_r][adj_c] = Some((r, c));
        dfs((adj_r, adj_c), map, visited)
    })
}
