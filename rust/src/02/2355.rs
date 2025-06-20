use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [a, b] = [(); 2].map(|_| input.next().unwrap());
    let (a, b) = (a.min(b), a.max(b));
    let sigma = |n: i64| n * (n + 1) / 2;

    println!("{}", sigma(b) - sigma(a - 1));
}
