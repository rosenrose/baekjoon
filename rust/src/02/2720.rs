use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    for mut cents in input.skip(1) {
        let mut counts = [0, 0, 0, 0];
        let values = [25, 10, 5, 1];

        for (i, value) in counts.iter_mut().enumerate() {
            *value += cents / values[i];
            cents %= values[i];
        }

        writeln!(
            output,
            "{} {} {} {}",
            counts[0], counts[1], counts[2], counts[3]
        )
        .unwrap();
    }

    print!("{output}");
}
