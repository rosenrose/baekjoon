use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut heap = BinaryHeap::with_capacity(n);

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        for num in buf.split_ascii_whitespace().flat_map(str::parse::<i32>) {
            if heap.len() < n {
                heap.push(Reverse(num));
                continue;
            }

            if num > heap.peek().unwrap().0 {
                heap.pop();
                heap.push(Reverse(num));
            }
        }
    }

    println!("{}", heap.peek().unwrap().0);
}
