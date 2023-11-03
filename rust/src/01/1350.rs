use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u64>);

    let cluster = input.next_back().unwrap();
    let count: u64 = (0..input.next().unwrap())
        .map(|_| input.next().unwrap().div_ceil(cluster))
        .sum();

    println!("{}", cluster * count);
}
