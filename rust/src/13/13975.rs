use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap() as usize;
        let mut nums: BinaryHeap<_> = input.by_ref().take(n).map(Reverse).collect();
        let mut cost = 0;

        while let (Some(Reverse(a)), Some(Reverse(b))) = (nums.pop(), nums.pop()) {
            let merged = a + b;

            cost += merged;
            nums.push(Reverse(merged));
        }

        println!("{cost}");
    }
}
