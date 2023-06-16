use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width, k] = [(); 3].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let map: Vec<Vec<_>> = input
        .map(|row| row.chars().map(|ch| ch == 'T').collect())
        .collect();

    let (start, end) = ((height - 1, 0), (0, width - 1));
    let mut visited = vec![vec![false; width]; height];
    visited[start.0][start.1] = true;

    let count = dfs(&map, &mut visited, start, end, 1, k as i32);

    println!("{count}");
}

fn dfs(
    map: &[Vec<bool>],
    visited: &mut Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
    dist: i32,
    k: i32,
) -> i32 {
    if start == end {
        return i32::from(dist == k);
    }

    let (width, height) = (map[0].len(), map.len());
    let adjacents = [
        (start.0.saturating_sub(1), start.1),
        (start.0, start.1.saturating_sub(1)),
        ((start.0 + 1).min(height - 1), start.1),
        (start.0, (start.1 + 1).min(width - 1)),
    ]
    .into_iter()
    .filter(|&adj| adj != start);

    adjacents
        .map(|(adj_r, adj_c)| {
            if visited[adj_r][adj_c] || map[adj_r][adj_c] {
                return 0;
            }

            visited[adj_r][adj_c] = true;

            let result = dfs(map, visited, (adj_r, adj_c), end, dist + 1, k);
            visited[adj_r][adj_c] = false;

            result
        })
        .sum()
}
