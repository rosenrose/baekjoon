use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let digit_cycle: HashMap<_, _> = (0..=9)
        .map(|i| {
            let mut cycle = vec![i];
            let mut next = (i * i) % 10;

            loop {
                if cycle.contains(&next) {
                    break;
                }

                cycle.push(next);
                next = (next * i) % 10;
            }

            (i, cycle)
        })
        .collect();

    for (a, b) in (0..input()).map(|_| (input(), input())) {
        let cycle = digit_cycle.get(&(a % 10)).unwrap();
        let number = cycle[(b - 1) % cycle.len()];

        writeln!(output, "{}", if number == 0 { 10 } else { number }).unwrap();
    }

    print!("{output}");
}
