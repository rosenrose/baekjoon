use std::io;

const WIDTH_MAX: usize = 50;
const HEIGHT_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let [width, height, k] = [(); 3].map(|_| input());
        let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

        for (x, y) in (0..k).map(|_| (input(), input())) {
            map[y][x] = true;
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

                while let Some((r, c)) = stack.pop() {
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

                count += 1;
            }
        }

        println!("{count}");
    }
}
