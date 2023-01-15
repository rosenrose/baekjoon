use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let mut heap = BinaryHeap::new();

    for x in input.skip(1) {
        match x {
            0 => {
                let (_, smallest) = heap.pop().unwrap_or(Reverse((0, 0))).0;
                writeln!(output, "{smallest}").unwrap();
            }
            x => heap.push(Reverse((x.abs(), x))),
        }
    }

    print!("{output}");
}
