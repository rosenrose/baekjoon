use std::io;

const WIDTH_MAX: usize = 50;
const HEIGHT_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.map(str::as_bytes).enumerate() {
        for (c, &num) in row.iter().enumerate() {
            map[r][c] = num;
        }
    }

    println!(
        "{}",
        if dfs(&map[..height], width) {
            "Yes"
        } else {
            "No"
        }
    );
}

fn dfs(map: &[[u8; WIDTH_MAX]], width: usize) -> bool {
    let height = map.len();
    let mut visited = [[None; WIDTH_MAX]; HEIGHT_MAX];

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
