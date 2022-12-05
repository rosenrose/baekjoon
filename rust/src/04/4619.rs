use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap());

    loop {
        let (b, n) = (input.next().unwrap(), input.next().unwrap());

        if (b, n) == (0, 0) {
            return;
        }

        let a = (b as f64).powf(1.0 / n as f64) as u32;
        let a = if a.pow(n).abs_diff(b) < (a + 1).pow(n).abs_diff(b) {
            a
        } else {
            a + 1
        };

        println!("{a}");
    }
}
