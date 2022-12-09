use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let (mut sum, mut min) = (0, 100);

        (0..7)
            .filter_map(|_| {
                let num = input.next().unwrap();
                (num % 2 == 0).then(|| num)
            })
            .for_each(|num| {
                sum += num;
                min = num.min(min);
            });

        writeln!(output, "{sum} {min}").unwrap();
    }

    print!("{output}");
}
