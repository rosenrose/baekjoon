use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for _ in 0..parse_int(input()) {
        let n = parse_int(input());

        let clothes = (0..n).fold(HashMap::new(), |mut acc, _| {
            let (_, category) = (input(), input());
            acc.entry(category).and_modify(|c| *c += 1).or_insert(1);

            acc
        });

        let product: i32 = clothes.values().map(|c| c + 1).product();

        println!("{}", product - 1);
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
