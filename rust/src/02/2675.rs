use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    for _ in 0..parse_int(input.next().unwrap()) {
        let r = parse_int(input.next().unwrap());
        let p: String = input
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().repeat(r))
            .collect();

        println!("{p}");
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
