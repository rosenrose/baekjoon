use std::collections::VecDeque;
use std::io;

enum Cells {
    Empty,
    Raw,
    Ripen,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i16>);

    let (width, height) = (input.next().unwrap(), input.next().unwrap());
    let mut queue = VecDeque::new();
    let mut raw_count = 0;

    let mut tomatoes: Vec<Vec<_>> = (0..height)
        .map(|r| {
            input
                .by_ref()
                .take(width as usize)
                .enumerate()
                .map(|(c, num)| match num {
                    1 => {
                        queue.push_back(((r, c as i16), 0));
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
        .collect();

    let mut time = 0;

    while let Some(((r, c), t)) = queue.pop_front() {
        time = t.max(time);

        let adjacents = [
            ((r - 1).max(0), c),
            (r, (c - 1).max(0)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if !matches!(tomatoes[adj_r as usize][adj_c as usize], Cells::Raw) {
                continue;
            }

            tomatoes[adj_r as usize][adj_c as usize] = Cells::Ripen;
            raw_count -= 1;
            queue.push_back(((adj_r, adj_c), t + 1));
        }
    }

    println!("{}", if raw_count == 0 { time } else { -1 });
}
