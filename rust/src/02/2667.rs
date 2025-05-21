use std::io;

const MAX: usize = 25;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut map = [[0; MAX]; MAX];

    for (r, row) in input.map(str::as_bytes).enumerate() {
        for (c, &num) in row.iter().enumerate() {
            map[r][c] = num;
        }
    }

    let (mut house_counts, house_counts_len) = get_house_counts(&map, n);
    house_counts[..house_counts_len].sort();

    println!("{}", house_counts_len);

    for count in &house_counts[..house_counts_len] {
        println!("{count}");
    }
}

fn get_house_counts(map: &[[u8; MAX]], n: usize) -> ([i32; (MAX * MAX).div_ceil(2)], usize) {
    let mut house_counts = [0; (MAX * MAX).div_ceil(2)];
    let mut house_counts_len = 0;
    let mut visited = [[false; MAX]; MAX];

    let is_pass = |r: usize, c: usize, visited: &[[bool; MAX]]| map[r][c] == b'0' || visited[r][c];

    for y in 0..n {
        for x in 0..n {
            if is_pass(y, x, &visited) {
                continue;
            }

            visited[y][x] = true;
            let mut count = 1;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(n - 1), c),
                    (r, (c + 1).min(n - 1)),
                ];

                for (adj_r, adj_c) in adjacents {
                    if is_pass(adj_r, adj_c, &visited) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                    count += 1;
                }
            }

            house_counts[house_counts_len] = count;
            house_counts_len += 1;
        }
    }

    (house_counts, house_counts_len)
}
