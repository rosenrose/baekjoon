use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let prices = [350.34, 230.90, 190.55, 125.30, 180.90];

    for _ in 0..input.next().unwrap() as i32 {
        let cost: f64 = prices
            .iter()
            .map(|price| price * input.next().unwrap())
            .sum();

        println!("${cost:.2}");
    }
}
