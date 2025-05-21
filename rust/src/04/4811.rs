use std::fmt::Write;
use std::io;

const MAX: usize = 30 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let mut memo = [0; MAX];
    memo[0..3].copy_from_slice(&[1, 1, 2]);

    for i in 3..=30 {
        memo[i] = (0..i).map(|j| memo[j] * memo[i - 1 - j]).sum::<i64>();
    }
    // println!("{memo:?}");
    for n in input.take_while(|&n| n != 0) {
        writeln!(output, "{}", memo[n]).unwrap();
    }

    print!("{output}");
}
