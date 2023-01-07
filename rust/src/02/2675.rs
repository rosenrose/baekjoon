use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
