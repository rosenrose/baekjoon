use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (mut under, mut over) = (BinaryHeap::new(), BinaryHeap::new());
    under.push(input.by_ref().skip(1).next().unwrap());

    println!("{}", under.peek().unwrap());

    for num in input {
        if num < *under.peek().unwrap() {
            under.push(num);
        } else {
            over.push(Reverse(num));
        }
        // #[rustfmt::skip]
        // println!("{:?} {:?}", under.clone().into_sorted_vec(), over.clone().into_sorted_vec());
        match (under.len(), over.len()) {
            (a, b) if a < b => under.push(over.pop().unwrap().0),
            (a, b) if a > b + 1 => over.push(Reverse(under.pop().unwrap())),
            _ => (),
        }

        writeln!(output, "{}", under.peek().unwrap()).unwrap();
    }

    print!("{output}");
}
