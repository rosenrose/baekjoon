use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    for _ in 0..parse_int(input.next().unwrap()) {
        let n = parse_int(input.next().unwrap());

        let clothes = (0..n).fold(HashMap::new(), |mut acc, _| {
            let (_, category) = (input.next().unwrap(), input.next().unwrap());
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
