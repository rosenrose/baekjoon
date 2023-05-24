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

    let (width, height) = (input.next().unwrap(), input.next().unwrap());
    let mut queue = VecDeque::new();

    let mut tomatoes: Vec<Vec<_>> = (0..height)
        .map(|r| {
            input
                .by_ref()
                .take(width as usize)
                .enumerate()
                .map(|(c, num)| match num {
                    1 => {
                        queue.push_back(((r, c as i32), 0));
                        Cells::Ripen
                    }
                    0 => Cells::Raw,
                    -1 => Cells::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut time = 0;

    while let Some(((r, c), t)) = queue.pop_front() {
        let adjacents = [
            ((r - 1).max(0), c),
            (r, (c - 1).max(0)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if tomatoes[adj_r as usize][adj_c as usize] != Cells::Raw {
                continue;
            }

            tomatoes[adj_r as usize][adj_c as usize] = Cells::Ripen;
            time = time.max(t + 1);
            queue.push_back(((adj_r, adj_c), t + 1));
        }
    }

    if tomatoes.iter().flatten().any(|cell| cell == &Cells::Raw) {
        println!("-1");
        return;
    }

    println!("{time}");
}
