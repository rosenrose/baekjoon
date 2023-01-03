use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f32>().unwrap());
    let mut input = || input.next().unwrap();

    let (bc, ca, ab) = (input(), input(), input());
    let (af, bd) = (input(), input());

    let (fb, dc) = (ab - af, bc - bd);
    let (m, n) = (af * bd, fb * dc);

    println!("{}", n * ca / (m + n));
}
// https://ko.wikipedia.org/wiki/체바_정리
