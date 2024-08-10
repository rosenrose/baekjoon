use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);
    let mut output = String::new();

    const MAX: usize = 1_000_000;
    let mut divisor_sums = [0; MAX + 1];

    for i in 1..=MAX {
        for j in (i..=MAX).step_by(i) {
            divisor_sums[j] += i;
        }

        divisor_sums[i] += divisor_sums[i - 1];
    }
    // println!("{divisor_sums:?}");
    for n in input.skip(1) {
        writeln!(output, "{}", divisor_sums[n]).unwrap();
    }

    print!("{output}");
}
