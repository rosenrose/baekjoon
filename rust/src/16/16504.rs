use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let sum: usize = (0..n).map(|_| input.by_ref().take(n).sum::<usize>()).sum();

    println!("{sum}");
}
