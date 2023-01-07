use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let is_3 = |bmi: f64| bmi >= 16.0 && bmi < 35.0;
    let is_4 = |bmi: f64| bmi < 16.0 || bmi >= 35.0;

    for (height, weight) in (0..input()).map(|_| (input(), input())) {
        let bmi = (weight * 10000) as f64 / (height * height) as f64;

        writeln!(
            output,
            "{}",
            match (height, bmi) {
                (1..=140, _) => 6,
                (141..=145, _) => 5,
                (146..=158, _) => 4,
                (159..=160, b) if is_3(b) => 3,
                (159..=160, b) if is_4(b) => 4,
                (161..=203, b) if b >= 20.0 && b < 25.0 => 1,
                (161..=203, b) if (b >= 18.5 && b < 20.0) || (b >= 25.0 && b < 30.0) => 2,
                (161..=203, b) if is_3(b) || (b >= 30.0 && b < 35.0) => 3,
                (161..=203, b) if is_4(b) => 4,
                (204.., _) => 4,
                _ => 0,
            }
        )
        .unwrap();
    }

    print!("{output}");
}
