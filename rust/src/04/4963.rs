use std::io;

const WIDTH_MAX: usize = 50;
const HEIGHT_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    while let (Some(width @ 1..), Some(height @ 1..)) = (input.next(), input.next()) {
        let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

        for r in 0..height {
            for (c, num) in input.by_ref().take(width).enumerate() {
                map[r][c] = num == 1;
            }
        }

        let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
        let mut count = 0;

        let is_pass =
            |r: usize, c: usize, visited: &[[bool; WIDTH_MAX]]| visited[r][c] || !map[r][c];

        for y in 0..height {
            for x in 0..width {
                if is_pass(y, x, &visited) {
                    continue;
                }

                visited[y][x] = true;
                let mut stack = vec![(y, x)];

                while let Some((row, col)) = stack.pop() {
                    let (up, down, left, right) = (
                        row.saturating_sub(1),
                        (row + 1).min(height - 1),
                        col.saturating_sub(1),
                        (col + 1).min(width - 1),
                    );
                    let adjacents = [
                        (up, left),
                        (up, col),
                        (up, right),
                        (row, left),
                        (row, right),
                        (down, left),
                        (down, col),
                        (down, right),
                    ];

                    for (adj_r, adj_c) in adjacents {
                        if is_pass(adj_r, adj_c, &visited) {
                            continue;
                        }

                        visited[adj_r][adj_c] = true;
                        stack.push((adj_r, adj_c));
                    }
                }

                count += 1;
            }
        }

        println!("{count}");
    }
}
