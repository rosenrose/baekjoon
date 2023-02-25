use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() as i32 {
        let (n, d, a, b, f) = (input() as i32, input(), input(), input(), input());

        println!("{n} {:.2}", d / (a + b) * f);
    }
}
