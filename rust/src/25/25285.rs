use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let is_1 = |bmi: f64| 20.0 <= bmi && bmi < 25.0;
    let is_2 = |bmi: f64| (18.5 <= bmi && bmi < 20.0) || (25.0 <= bmi && bmi < 30.0);
    let is_3 = |bmi: f64| (16.0 <= bmi && bmi < 18.5) || (30.0 <= bmi && bmi < 35.0);
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
                (159..=160, b) if is_4(b) => 4,
                (159..=160, b) if !is_4(b) => 3,
                (161..=203, b) if is_1(b) => 1,
                (161..=203, b) if is_2(b) => 2,
                (161..=203, b) if is_3(b) => 3,
                (161..=203, b) if is_4(b) => 4,
                (204.., _) => 4,
                _ => unreachable!(),
            }
        )
        .unwrap();
    }

    print!("{output}");
}
