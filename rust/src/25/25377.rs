use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut min_time = i32::MAX;

    for _ in 0..input.next().unwrap() {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());

        if a > b {
            continue;
        }

        let time = a.max(b);
        min_time = time.min(min_time);
    }

    println!("{}", if min_time == i32::MAX { -1 } else { min_time });
}
