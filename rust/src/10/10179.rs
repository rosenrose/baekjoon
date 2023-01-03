use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<f32>().unwrap());

    for price in input.skip(1) {
        println!("${:.2}", price * 0.8);
    }
}
