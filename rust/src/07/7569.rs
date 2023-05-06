use std::collections::VecDeque;
use std::io;

#[derive(PartialEq)]
enum Cells {
    Empty,
    Raw,
    Ripen,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (m, n, k) = (input() as usize, input() as usize, input() as usize);
    let mut ripens = Vec::new();
    let mut raw_count = 0;

    let mut tomatoes: Vec<Vec<Vec<_>>> = (0..k)
        .map(|h| {
            (0..n)
                .map(|r| {
                    (0..m)
                        .map(|c| match input() {
                            1 => {
                                ripens.push((h, r, c));
                                Cells::Ripen
                            }
                            0 => {
                                raw_count += 1;
                                Cells::Raw
                            }
                            -1 => Cells::Empty,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut queue: VecDeque<_> = ripens.iter().map(|&coord| (coord, 0)).collect();
    let mut time = 0;

    while let Some(((h, r, c), t)) = queue.pop_front() {
        let adjacents = [
            (h.saturating_sub(1), r, c),
            (h, r.saturating_sub(1), c),
            (h, r, c.saturating_sub(1)),
            ((h + 1).min(k - 1), r, c),
            (h, (r + 1).min(n - 1), c),
            (h, r, (c + 1).min(m - 1)),
        ];

        for &(adj_h, adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (h, r, c)) {
            if tomatoes[adj_h][adj_r][adj_c] != Cells::Raw {
                continue;
            }

            tomatoes[adj_h][adj_r][adj_c] = Cells::Ripen;
            raw_count -= 1;
            time = time.max(t + 1);
            queue.push_back(((adj_h, adj_r, adj_c), t + 1));
        }
    }

    println!("{}", if raw_count == 0 { time } else { -1 });
}
