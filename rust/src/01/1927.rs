use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut heap = BinaryHeap::with_capacity(n >> 1);

    for x in input {
        match x {
            0 => writeln!(output, "{}", heap.pop().unwrap_or(Reverse(0)).0).unwrap(),
            _ => heap.push(Reverse(x)),
        }
    }

    print!("{output}");
}
