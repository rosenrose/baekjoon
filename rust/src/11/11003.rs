use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (n, size) = (input.next().unwrap() as usize, input.next().unwrap());
    let mut heap = BinaryHeap::with_capacity(n >> 1);

    for (i, num) in input.enumerate() {
        heap.push(Reverse((num, i as i32)));

        loop {
            let (_, min_idx) = heap.peek().unwrap().0;

            if min_idx < (i as i32 + 1) - size {
                heap.pop();
            } else {
                break;
            }
        }

        write!(output, "{} ", heap.peek().unwrap().0 .0).unwrap();
    }

    print!("{output}");
}
