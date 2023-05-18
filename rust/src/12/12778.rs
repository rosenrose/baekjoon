use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let offset = b'A';

    for _ in 0..parse_int(input()) {
        let n = parse_int(input());

        match input() {
            "C" => {
                for ch in (0..n).map(|_| input().as_bytes()[0]) {
                    print!("{} ", ch - offset + 1);
                }
            }
            "N" => {
                for num in (0..n).map(|_| parse_int(input())) {
                    print!("{} ", (num as u8 + offset - 1) as char);
                }
            }
            _ => unreachable!(),
        }

        println!("");
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
