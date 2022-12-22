use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let mut heap = BinaryHeap::new();

    for x in input.skip(1) {
        match x {
            0 => {
                let (_, smallest) = heap.pop().unwrap_or((Reverse(0), Reverse(0)));
                writeln!(output, "{}", smallest.0).unwrap();
            }
            x => heap.push((Reverse(x.abs()), Reverse(x))),
        }
    }

    print!("{output}");
}
