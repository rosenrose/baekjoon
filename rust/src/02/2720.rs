use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let values = [25, 10, 5, 1];

    for mut cents in input.skip(1) {
        let mut counts = [0, 0, 0, 0];

        for i in 0..counts.len() {
            counts[i] += cents / values[i];
            cents %= values[i];
        }

        let [quarter, dime, nickel, penny] = counts;

        writeln!(output, "{quarter} {dime} {nickel} {penny}").unwrap();
    }

    print!("{output}");
}
