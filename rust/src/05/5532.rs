use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    if let [l, a, b, c, d] = input.collect::<Vec<_>>()[..] {
        let lang_count = (a as f64 / c as f64).ceil() as i32;
        let math_count = (b as f64 / d as f64).ceil() as i32;

        println!("{}", l - lang_count.max(math_count));
    }
}
