use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let map: Vec<_> = buf.lines().skip(1).map(str::as_bytes).collect();

    let mut houses = get_houses(&map);
    houses.sort();

    println!("{}", houses.len());

    for house in houses {
        println!("{house}");
    }
}

fn get_houses(map: &Vec<&[u8]>) -> Vec<i32> {
    let n = map.len();
    let mut houses = Vec::new();
    let mut visited = vec![vec![false; n]; n];

    let is_pass =
        |r: usize, c: usize, visited: &Vec<Vec<bool>>| map[r][c] as char == '0' || visited[r][c];

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

                for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
                    if is_pass(adj_r, adj_c, &visited) {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                    count += 1;
                }
            }

            houses.push(count);
        }
    }

    houses
}
