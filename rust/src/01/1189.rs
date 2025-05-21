use std::io;

const WIDTH_MAX: usize = 5;
const HEIGHT_MAX: usize = 5;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width, k] = [(); 3].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.enumerate() {
        for (c, ch) in row.char_indices() {
            map[r][c] = ch == 'T';
        }
    }

    let (start, end) = ((height - 1, 0), (0, width - 1));
    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
    visited[start.0][start.1] = true;

    let count = dfs(&map[..height], width, &mut visited, start, end, 1, k as i32);

    println!("{count}");
}

fn dfs(
    map: &[[bool; WIDTH_MAX]],
    width: usize,
    visited: &mut [[bool; WIDTH_MAX]],
    start: (usize, usize),
    end: (usize, usize),
    dist: i32,
    k: i32,
) -> i32 {
    if start == end {
        return (dist == k) as i32;
    }

    let height = map.len();
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

            let result = dfs(map, width, visited, (adj_r, adj_c), end, dist + 1, k);
            visited[adj_r][adj_c] = false;

            result
        })
        .sum()
}
