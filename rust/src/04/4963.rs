use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    while let (Some(w @ 1..), Some(h @ 1..)) = (input.next(), input.next()) {
        let map: Vec<Vec<_>> = (0..h).map(|_| input.by_ref().take(w).collect()).collect();
        let mut visited = vec![vec![false; w]; h];
        let mut count = 0;

        let is_pass = |r: usize, c: usize, visited: &[Vec<bool>]| visited[r][c] || map[r][c] == 0;

        for y in 0..h {
            for x in 0..w {
                if is_pass(y, x, &visited) {
                    continue;
                }

                visited[y][x] = true;
                let mut stack = vec![(y, x)];

                while let Some((row, col)) = stack.pop() {
                    let (up, down, left, right) = (
                        row.saturating_sub(1),
                        (row + 1).min(h - 1),
                        col.saturating_sub(1),
                        (col + 1).min(w - 1),
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

                    for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (row, col)) {
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
