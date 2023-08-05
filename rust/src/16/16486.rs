use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<f64>);

    let [d1, d2] = [(); 2].map(|_| input.next().unwrap());
    const PI: f64 = 3.141592;

    println!("{:.6}", (d1 * 2.0) + (2.0 * PI * d2));
}
