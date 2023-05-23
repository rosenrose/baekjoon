use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let picture: Vec<_> = buf.lines().skip(1).map(str::as_bytes).collect();

    let normal_count = get_count(&picture, false);
    let blind_count = get_count(&picture, true);

    println!("{normal_count} {blind_count}");
}

fn get_count(picture: &[&[u8]], is_blind: bool) -> i32 {
    let n = picture.len();
    let mut visited = vec![vec![false; n]; n];
    let mut count = 0;

    for y in 0..n {
        for x in 0..n {
            if visited[y][x] {
                continue;
            }

            visited[y][x] = true;
            let color = picture[y][x];
            let mut stack = vec![(y, x)];

            while let Some((r, c)) = stack.pop() {
                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(n - 1), c),
                    (r, (c + 1).min(n - 1)),
                ];

                for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
                    if visited[adj_r][adj_c] {
                        continue;
                    }

                    let adj_color = picture[adj_r][adj_c];

                    if !is_blind && (adj_color != color) {
                        continue;
                    }
                    if is_blind
                        && matches!(
                            (adj_color as char, color as char),
                            ('R' | 'G', 'B') | ('B', 'R' | 'G')
                        )
                    {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    stack.push((adj_r, adj_c));
                }
            }

            count += 1;
        }
    }

    count
}
