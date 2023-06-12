use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let n = parse_int(input.next().unwrap());

    'outer: for [p, len, arr] in (0..n).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        let mut arr = if len != "0" {
            arr[1..arr.len() - 1].split(',').map(parse_int).collect()
        } else {
            VecDeque::new()
        };

        let mut is_reverse = false;

        for func in p.as_bytes() {
            match func {
                b'R' => is_reverse = !is_reverse,
                b'D' => {
                    let removed = if is_reverse {
                        arr.pop_back()
                    } else {
                        arr.pop_front()
                    };

                    if removed.is_none() {
                        writeln!(output, "error").unwrap();
                        continue 'outer;
                    }
                }
                _ => unreachable!(),
            };
        }

        let mut arr = Vec::from_iter(arr);

        if is_reverse {
            arr.reverse();
        }

        writeln!(output, "{}", format!("{arr:?}").replace(' ', "")).unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
