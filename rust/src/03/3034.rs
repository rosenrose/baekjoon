use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut input = || input.next().unwrap();

    let (n, w, h) = (input() as i32, input(), input());

    for len in (0..n).map(|_| input()) {
        println!("{}", if len <= w.hypot(h) { "DA" } else { "NE" });
    }
}
