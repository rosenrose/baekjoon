use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let (n, mut p) = (input.next().unwrap(), input.next().unwrap());
    let discounts = [
        0,
        500,
        (p as f64 * 0.1) as i32,
        2000,
        (p as f64 * 0.25) as i32,
    ];

    p -= discounts[..=(n / 5).min(4) as usize]
        .into_iter()
        .max()
        .unwrap();

    println!("{}", p.max(0));
}
