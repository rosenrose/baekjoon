use std::io;

const MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let [n, k, r] = [(); 3].map(|_| input());
    let mut map = [(); MAX].map(|_| [(); MAX].map(|_| (Vec::with_capacity(4), false)));

    for [r1, c1, r2, c2] in (0..r).map(|_| [(); 4].map(|_| input() - 1)) {
        map[r1][c1].0.push((r2, c2));
        map[r2][c2].0.push((r1, c1));
    }

    for (r, c) in (0..k).map(|_| (input() - 1, input() - 1)) {
        map[r][c].1 = true;
    }

    let mut visited = [[false; MAX]; MAX];
    let mut groups = [0; MAX * MAX / 2];
    let mut groups_len = 0;

    for y in 0..n {
        for x in 0..n {
            if visited[y][x] {
                continue;
            }

            visited[y][x] = true;
            let mut count = 0;
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                if map[r][c].1 {
                    count += 1;
                }

                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(n - 1), c),
                    (r, (c + 1).min(n - 1)),
                ]
                .into_iter()
                .filter(|adj| !map[r][c].0.contains(adj));

                for (adj_r, adj_c) in adjacents {
                    if visited[adj_r][adj_c] {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            if count > 0 {
                groups[groups_len] = count;
                groups_len += 1;
            }
        }
    }

    let mut pairs = 0;

    for a in 0..groups_len - 1 {
        for b in a + 1..groups_len {
            pairs += groups[a] * groups[b];
        }
    }

    println!("{pairs}");
}
