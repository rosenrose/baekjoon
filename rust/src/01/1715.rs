use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let mut nums: BinaryHeap<_> = input.skip(1).map(Reverse).collect();
    let mut count = 0;

    while let (Some(Reverse(a)), Some(Reverse(b))) = (nums.pop(), nums.pop()) {
        let merged = a + b;

        count += merged;
        nums.push(Reverse(merged));
    }

    println!("{count}");
}
