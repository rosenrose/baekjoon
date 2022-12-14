use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for _ in 0..parse_int(input()) {
        let n = parse_int(input());

        let (max_school, _) = (0..n)
            .map(|_| (input(), parse_int(input())))
            .max_by_key(|&(_, alcohol)| alcohol)
            .unwrap();

        println!("{max_school}");
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.trim().parse().unwrap()
}
