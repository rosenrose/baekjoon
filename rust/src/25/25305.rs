use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    input.next();

    let k = input.next().unwrap();
    let mut arr: Vec<_> = input.collect();

    arr.sort_by(|a, b| b.cmp(a));

    println!("{}", arr[k - 1]);
}
