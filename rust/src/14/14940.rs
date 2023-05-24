use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i16>);
    let mut output = String::new();

    let (height, width) = (input.next().unwrap(), input.next().unwrap());
    let mut start = (0, 0);
    let mut map: Vec<Vec<_>> = (0..height)
        .map(|r| {
            input
                .by_ref()
                .take(width as usize)
                .enumerate()
                .map(|(c, num)| match num {
                    0 => 0_i16,
                    1 => -1,
                    2 => {
                        start = (r, c as i16);
                        0
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some(((r, c), dist)) = queue.pop_front() {
        let adjacents = [
            ((r - 1).max(0), c),
            (r, (c - 1).max(0)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
            if map[adj_r as usize][adj_c as usize] != -1 {
                continue;
            }

            let new_dist = dist + 1;
            map[adj_r as usize][adj_c as usize] = new_dist;
            queue.push_back(((adj_r, adj_c), new_dist));
        }
    }

    for row in map {
        for dist in row {
            write!(output, "{dist} ").unwrap();
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}
