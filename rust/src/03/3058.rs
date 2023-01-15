use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
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
