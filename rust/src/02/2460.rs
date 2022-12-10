use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    const FULL: i32 = 10000;
    let mut max_count = 0;

    let _final_count = (0..10)
        .map(|_| (input.next().unwrap(), input.next().unwrap()))
        .fold(0, |current, (off, on)| {
            let next = (current - off + on).min(FULL);
            max_count = next.max(max_count);

            next
        });

    println!("{max_count}");
}
