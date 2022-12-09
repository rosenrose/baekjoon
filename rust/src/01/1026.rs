use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let mut a: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
    let mut b: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();

    a.sort();
    b.sort_by(|x, y| y.cmp(x));

    let product_sum: i32 = a.iter().zip(b).map(|(a, b)| a * b).sum();

    println!("{product_sum}");
}
