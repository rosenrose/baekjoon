use std::io;

const MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let (mut a, mut b) = ([0; MAX], [0; MAX]);

    for (i, num) in input.by_ref().take(n).enumerate() {
        a[i] = num;
    }
    for (i, num) in input.enumerate() {
        b[i] = num;
    }

    a[..n].sort();
    b[..n].sort();

    let product_sum: usize = a[..n]
        .iter()
        .zip(b[..n].iter().rev())
        .map(|(a, b)| a * b)
        .sum();

    println!("{product_sum}");
}
