use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (mut under, mut over) = (BinaryHeap::new(), BinaryHeap::new());

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap() as usize;
        let mut mid_values = Vec::with_capacity((n + 1) >> 1);

        under.push(input.next().unwrap());
        mid_values.push(*under.peek().unwrap());

        for (i, num) in input.by_ref().take(n - 1).enumerate() {
            if num < *under.peek().unwrap() {
                under.push(num);
            } else {
                over.push(Reverse(num));
            }

            match (under.len(), over.len()) {
                (a, b) if a < b => under.push(over.pop().unwrap().0),
                (a, b) if a > b + 1 => over.push(Reverse(under.pop().unwrap())),
                _ => (),
            }

            if i & 1 == 1 {
                mid_values.push(*under.peek().unwrap());
            }
        }

        writeln!(output, "{}", mid_values.len()).unwrap();

        for chunk in mid_values.chunks(10) {
            for num in chunk {
                write!(output, "{num} ").unwrap();
            }
            writeln!(output, "").unwrap();
        }

        under.clear();
        over.clear();
    }

    print!("{output}");
}
