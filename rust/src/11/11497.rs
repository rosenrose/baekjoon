use std::collections::VecDeque;
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
        let n = input.next().unwrap();

        let mut logs: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
        logs.sort_unstable();

        let mut new_logs = VecDeque::new();

        for log in logs {
            if new_logs.len() % 2 == 0 {
                new_logs.push_back(log);
            } else {
                new_logs.push_front(log);
            }
        }

        let len = new_logs.len();
        let mut max_diff = new_logs[0].abs_diff(new_logs[len - 1]);

        for i in 1..len {
            max_diff = new_logs[i - 1].abs_diff(new_logs[i]).max(max_diff);
        }

        writeln!(output, "{max_diff}").unwrap();
    }

    print!("{output}");
}
