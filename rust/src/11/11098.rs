use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for _ in 0..parse_int(input()) {
        let p = parse_int(input());
        let (_, vip) = (0..p)
            .map(|_| (parse_int(input()), input()))
            .max_by_key(|&(guarantee, _)| guarantee)
            .unwrap();

        println!("{vip}");
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
