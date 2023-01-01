use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    const CONSTANTS: [[f32; 3]; 7] = [
        [9.23076, 26.7, 1.835],
        [1.84523, 75.0, 1.348],
        [56.0211, 1.5, 1.05],
        [4.99087, 42.5, 1.81],
        [0.188807, 210.0, 1.41],
        [15.9803, 3.8, 1.04],
        [0.11193, 254.0, 1.88],
    ];

    for _ in 0..input.next().unwrap() {
        let score: i32 = CONSTANTS
            .iter()
            .enumerate()
            .map(|(i, &[a, b, c])| {
                let p = input.next().unwrap() as f32;

                (match i {
                    0 | 3 | 6 => a * (b - p).powf(c),
                    1 | 2 | 4 | 5 => a * (p - b).powf(c),
                    _ => 0.0,
                }) as i32
            })
            .sum();

        writeln!(output, "{score}").unwrap();
    }

    print!("{output}");
}