use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();

    for (s, p) in (0..n).map(|_| (input(), input())) {
        let copies = s.matches(p).count();

        println!("{}", s.replace(p, "").len() + copies);
    }
}
