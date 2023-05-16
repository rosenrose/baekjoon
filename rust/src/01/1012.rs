use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (m, n, k) = (input(), input(), input());
        let mut map = vec![vec![0; m]; n];

        for (x, y) in (0..k).map(|_| (input(), input())) {
            map[y][x] = 1;
        }

        let mut visited = vec![vec![false; m]; n];
        let mut count = 0;
        let is_pass = |r: usize, c: usize, visited: &[Vec<bool>]| map[r][c] == 0 || visited[r][c];

        for y in 0..n {
            for x in 0..m {
                if is_pass(y, x, &visited) {
                    continue;
                }

                visited[y][x] = true;
                let mut stack = vec![(y, x)];

                while let Some((r, c)) = stack.pop() {
                    let adjacents = [
                        (r.saturating_sub(1), c),
                        (r, c.saturating_sub(1)),
                        ((r + 1).min(n - 1), c),
                        (r, (c + 1).min(m - 1)),
                    ];

                    for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
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
