use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut a: Vec<_> = input.by_ref().take(n).collect();
    let mut b: Vec<_> = input.by_ref().take(n).collect();

    a.sort();
    b.sort();

    let product_sum: usize = a.iter().zip(b.iter().rev()).map(|(a, b)| a * b).sum();

    println!("{product_sum}");
}
