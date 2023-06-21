use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_whitespace();

    let map: Vec<Vec<_>> = input
        .skip(2)
        .map(|row| row.as_bytes().iter().map(|&ch| ch == b'x').collect())
        .collect();

    let max_count = dfs(&map);

    println!("{max_count}");
}

fn dfs(map: &[Vec<bool>]) -> i32 {
    let (width, height) = (map[0].len(), map.len());
    let mut count = 0;
    let mut visited = vec![vec![false; width]; height];

    for y in 0..height {
        let mut stack = vec![(y, 0)];

        while let Some((r, c)) = stack.pop() {
            if visited[r][c] {
                continue;
            }

            visited[r][c] = true;

            if c == width - 1 {
                count += 1;
                break;
            }

            let adjacents = [r as i32 - 1, r as i32, r as i32 + 1]
                .into_iter()
                .filter_map(|adj_r| {
                    (0 <= adj_r && adj_r < height as i32).then(|| (adj_r as usize, c + 1))
                });

            for (adj_r, adj_c) in adjacents.rev() {
                if map[adj_r][adj_c] {
                    continue;
                }

                stack.push((adj_r, adj_c));
            }
        }
    }

    count
}
