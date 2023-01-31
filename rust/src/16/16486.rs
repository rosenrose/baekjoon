use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<f32>);

    let (d1, d2) = (input.next().unwrap(), input.next().unwrap());
    const PI: f32 = 3.141592;

    println!("{:.6}", (d1 * 2.0) + (2.0 * PI * d2));
}
