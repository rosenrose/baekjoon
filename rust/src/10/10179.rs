use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().map(|s| s.parse::<f32>().unwrap());

    for price in input.skip(1) {
        println!("${:.2}", price * 0.8);
    }
}
