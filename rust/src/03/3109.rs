use std::io;

const WIDTH_MAX: usize = 500;
const HEIGHT_MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.map(str::as_bytes).enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            map[r][c] = ch == b'x';
        }
    }

    let max_count = dfs(&map[..height], width);

    println!("{max_count}");
}

fn dfs(map: &[[bool; WIDTH_MAX]], width: usize) -> i32 {
    let height = map.len();
    let mut count = 0;
    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];

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
