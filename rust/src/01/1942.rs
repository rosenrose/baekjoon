enum Time {
    H,
    M,
    S,
}

use std::io;
use Time::{H, M, S};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().map(parse_time);
    let (h, m, s) = (H as usize, M as usize, S as usize);

    for (mut start, end) in (0..3).map(|_| (input.next().unwrap(), input.next().unwrap())) {
        let mut count = 0;

        loop {
            if start.iter().sum::<i32>() % 3 == 0 {
                count += 1;
            }

            if start == end {
                break;
            }
            // println!("{start:?}");
            start[s] += 1;

            start[m] += start[s] / 60;
            start[s] %= 60;

            start[h] += start[m] / 60;
            start[m] %= 60;

            start[h] %= 24;
        }

        println!("{count}");
    }
}

fn parse_time(s: &str) -> Vec<i32> {
    s.split(':').flat_map(str::parse).collect()
}
