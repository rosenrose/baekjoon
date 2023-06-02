use std::collections::VecDeque;
use std::io;

enum Cells {
    Empty,
    Raw,
    Ripen,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (cols, rows, height) = (input(), input(), input());
    let mut queue = VecDeque::new();
    let mut raw_count = 0;

    let mut tomatoes: Vec<Vec<Vec<_>>> = (0..height)
        .map(|h| {
            (0..rows)
                .map(|r| {
                    (0..cols)
                        .map(|c| match input() {
                            1 => {
                                queue.push_back(((h, r, c), 0));
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

    let mut time = 0;

    while let Some(((h, r, c), t)) = queue.pop_front() {
        time = t.max(time);

        let adjacents = [
            ((h - 1).max(0), r, c),
            (h, (r - 1).max(0), c),
            (h, r, (c - 1).max(0)),
            ((h + 1).min(height - 1), r, c),
            (h, (r + 1).min(rows - 1), c),
            (h, r, (c + 1).min(cols - 1)),
        ];

        for adj in adjacents {
            if !matches!(
                tomatoes[adj.0 as usize][adj.1 as usize][adj.2 as usize],
                Cells::Raw
            ) {
                continue;
            }

            tomatoes[adj.0 as usize][adj.1 as usize][adj.2 as usize] = Cells::Ripen;
            raw_count -= 1;
            queue.push_back((adj, t + 1));
        }
    }

    println!("{}", if raw_count == 0 { time } else { -1 });
}
