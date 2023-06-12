use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let [k, d1, d2] = [(); 3].map(|_| input.next().unwrap());
    let d3 = (d1 - d2) / 2.0;

    println!("{}", k * k - d3 * d3);
}
