use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let map: Vec<Vec<_>> = input
        .map(|row| row.as_bytes().iter().map(|&ch| ch == b'x').collect())
        .collect();

    let mut visited = vec![vec![false; width]; height];
    let max_count: i32 = (0..height)
        .map(|r| i32::from(dfs((r, 0), &map, &mut visited)))
        .sum();

    println!("{max_count}");
}

fn dfs((r, c): (usize, usize), map: &[Vec<bool>], visited: &mut Vec<Vec<bool>>) -> bool {
    let (width, height) = (map[0].len(), map.len() as i32);
    visited[r][c] = true;

    if c == width - 1 {
        return true;
    }

    let adjacents = [r as i32 - 1, r as i32, r as i32 + 1]
        .into_iter()
        .filter_map(|adj_r| (0 <= adj_r && adj_r < height).then(|| (adj_r as usize, c + 1)));

    for (adj_r, adj_c) in adjacents {
        if visited[adj_r][adj_c] || map[adj_r][adj_c] {
            continue;
        }

        let is_connected = dfs((adj_r, adj_c), map, visited);

        if is_connected {
            return is_connected;
        }
    }

    false
}
