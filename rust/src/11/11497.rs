use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let n = input() as usize;
        let mut logs: Vec<_> = (0..n).map(|_| input()).collect();
        logs.sort_unstable();

        let logs = logs.iter().fold(VecDeque::new(), |mut acc, &log| {
            if acc.len() % 2 == 0 {
                acc.push_back(log);
            } else {
                acc.push_front(log);
            }

            acc
        });

        let max_diff = (1..n).fold(logs[0].abs_diff(logs[n - 1]), |max, i| {
            max.max(logs[i - 1].abs_diff(logs[i]))
        });

        writeln!(output, "{max_diff}").unwrap();
    }

    print!("{output}");
}
