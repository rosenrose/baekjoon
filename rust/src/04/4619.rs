use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    while let (Some(b @ 1..), Some(n @ 1..)) = (input.next(), input.next()) {
        let a = (b as f64).powf(1.0 / n as f64) as u32;
        let a = if a.pow(n).abs_diff(b) < (a + 1).pow(n).abs_diff(b) {
            a
        } else {
            a + 1
        };

        println!("{a}");
    }
}
