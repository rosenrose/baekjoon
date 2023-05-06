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

    let (m, n) = (
        input.next().unwrap() as usize,
        input.next().unwrap() as usize,
    );
    let mut ripens = Vec::new();

    let mut tomatoes: Vec<Vec<_>> = (0..n)
        .map(|r| {
            input
                .by_ref()
                .take(m)
                .enumerate()
                .map(|(c, num)| match num {
                    1 => {
                        ripens.push((r, c));
                        Cells::Ripen
                    }
                    0 => Cells::Raw,
                    -1 => Cells::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut queue: VecDeque<_> = ripens.iter().map(|&coord| (coord, 0)).collect();
    let mut time = 0;

    while let Some(((r, c), t)) = queue.pop_front() {
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(n - 1), c),
            (r, (c + 1).min(m - 1)),
        ];

        for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
            if tomatoes[adj_r][adj_c] != Cells::Raw {
                continue;
            }

            tomatoes[adj_r][adj_c] = Cells::Ripen;
            time = time.max(t + 1);
            queue.push_back(((adj_r, adj_c), t + 1));
        }
    }

    if tomatoes.iter().any(|row| row.contains(&Cells::Raw)) {
        println!("-1");
        return;
    }

    println!("{time}");
}
