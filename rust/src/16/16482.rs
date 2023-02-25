use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut input = || input.next().unwrap();

    let (bc, ca, ab) = (input(), input(), input());
    let (af, bd) = (input(), input());

    let (fb, dc) = (ab - af, bc - bd);
    let (m, n) = (af * bd, fb * dc);

    println!("{}", n * ca / (m + n));
}
// https://ko.wikipedia.org/wiki/체바_정리
