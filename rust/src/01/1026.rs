use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut a: Vec<_> = (0..n).map(|_| input()).collect();
    let mut b: Vec<_> = (0..n).map(|_| input()).collect();

    a.sort();
    b.sort_by(|x, y| y.cmp(x));

    let product_sum: i32 = a.iter().zip(b).map(|(a, b)| a * b).sum();

    println!("{product_sum}");
}
