use std::cmp::Reverse;
use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut num_counts = HashMap::with_capacity(n);

    for (i, num) in input.skip(1).enumerate() {
        num_counts
            .entry(num)
            .and_modify(|(count, _)| *count += 1)
            .or_insert((1, i));
    }

    let mut num_counts = Vec::from_iter(num_counts);
    num_counts.sort_by_key(|&(_, (count, idx))| (Reverse(count), idx));
    // println!("{num_counts:?}");
    for (num, (count, _)) in num_counts {
        for _ in 0..count {
            print!("{num} ");
        }
    }
}
