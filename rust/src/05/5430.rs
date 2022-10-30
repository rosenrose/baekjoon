use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};
use std::string::ToString;

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    'outer: for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let p = buf.trim().to_string();
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let len: i32 = buf.trim().parse().unwrap();
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let arr = buf.trim();
        let mut arr: VecDeque<i32> = if len > 0 {
            arr[1..arr.len() - 1]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        } else {
            VecDeque::new()
        };

        let mut is_reverse = false;

        for func in p.chars() {
            match func {
                'R' => is_reverse = !is_reverse,
                'D' => {
                    let removed = if is_reverse {
                        arr.pop_back()
                    } else {
                        arr.pop_front()
                    };

                    if let None = removed {
                        writeln!(stdout, "error").unwrap();
                        continue 'outer;
                    }
                }
                _ => (),
            };
        }

        let result: Vec<String> = if is_reverse {
            arr.iter().rev().map(ToString::to_string).collect()
        } else {
            arr.iter().map(ToString::to_string).collect()
        };

        writeln!(stdout, "[{}]", result.join(",")).unwrap();
    }
}
