use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i128>);

    for _ in 0..3 {
        let n = input.next().unwrap() as usize;
        let sum: i128 = input.by_ref().take(n).sum();

        println!(
            "{}",
            match sum.cmp(&0) {
                Ordering::Less => "-",
                Ordering::Equal => "0",
                Ordering::Greater => "+",
            }
        );
    }
}
