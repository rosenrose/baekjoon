use std::cmp::Ordering;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for (d, n, s, p) in (0..input()).map(|_| (input(), input(), input(), input())) {
        let serial = n * s;
        let parallel = d + n * p;

        writeln!(
            output,
            "{}",
            match serial.cmp(&parallel) {
                Ordering::Less => "do not parallelize",
                Ordering::Equal => "does not matter",
                Ordering::Greater => "parallelize",
            }
        )
        .unwrap();
    }

    print!("{output}");
}
