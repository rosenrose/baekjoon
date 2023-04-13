use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap() as usize;
        let mut logs: Vec<_> = input.by_ref().take(n).collect();
        logs.sort_unstable();

        let mut new_logs = VecDeque::with_capacity(n);

        for log in logs {
            if new_logs.len() & 1 == 0 {
                new_logs.push_back(log);
            } else {
                new_logs.push_front(log);
            }
        }

        let mut max_diff = new_logs[0].abs_diff(new_logs[n - 1]);

        for i in 1..n {
            max_diff = new_logs[i - 1].abs_diff(new_logs[i]).max(max_diff);
        }

        writeln!(output, "{max_diff}").unwrap();
    }

    print!("{output}");
}
