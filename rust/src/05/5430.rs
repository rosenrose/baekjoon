use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let n = parse_int(input.next().unwrap());

    'outer: for _ in 0..n {
        let p = input.next().unwrap();
        let len = parse_int(input.next().unwrap());
        let arr = input.next().unwrap();

        let mut arr: VecDeque<_> = if len > 0 {
            arr[1..arr.len() - 1].split(',').map(parse_int).collect()
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

                    if removed.is_none() {
                        writeln!(output, "error").unwrap();
                        continue 'outer;
                    }
                }
                _ => (),
            };
        }

        write!(output, "[").unwrap();

        let print_arr = |(i, num): (usize, &i32)| {
            if i > 0 {
                write!(output, ",").unwrap();
            }
            write!(output, "{num}").unwrap();
        };

        if is_reverse {
            arr.iter().rev().enumerate().for_each(print_arr);
        } else {
            arr.iter().enumerate().for_each(print_arr);
        };

        writeln!(output, "]").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
