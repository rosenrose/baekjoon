use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let [a, b, c, m] = [(); 4].map(|_| input.next().unwrap());
    let n = b - m;
    let x = (a * a - c * c + n * n - m * m) / (2.0 * (m + n));

    println!("{}", a * a - m * m - 2.0 * m * x);
}
