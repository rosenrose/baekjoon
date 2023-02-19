use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<f64>);

    for price in input.skip(1) {
        println!("${:.2}", price * 0.8);
    }
}
