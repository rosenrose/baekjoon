use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let x = input.next_back().unwrap();
    let mut nums: Vec<_> = input.skip(1).collect();
    nums.sort_unstable();

    let mid = nums.binary_search(&((x + 1) / 2)).unwrap_or_else(|i| i);
    let (left, right) = nums.split_at(mid);
    // println!("{left:?} {right:?}");
    let count = left
        .iter()
        .filter(|&num| right.binary_search(&(x - num)).is_ok())
        .count();

    println!("{count}");
}
