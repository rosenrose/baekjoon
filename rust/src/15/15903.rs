use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [_, m] = [(); 2].map(|_| input.next().unwrap());
    let mut nums: BinaryHeap<_> = input.map(Reverse).collect();

    for _ in 0..m {
        let (a, b) = (nums.pop().unwrap().0, nums.pop().unwrap().0);
        let sum = a + b;

        nums.push(Reverse(sum));
        nums.push(Reverse(sum));
    }

    println!("{}", nums.iter().map(|n| n.0).sum::<i64>());
}
