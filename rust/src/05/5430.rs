use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let n: i32 = input.next().unwrap().parse().unwrap();

    'outer: for [p, len, arr] in (0..n).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        let mut arr = if len != "0" {
            arr[1..arr.len() - 1]
                .split(',')
                .flat_map(str::parse::<u8>)
                .collect()
        } else {
            VecDeque::new()
        };

        let mut is_reverse = false;

        for op in p.as_bytes() {
            match op {
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

        if is_reverse {
            print_arr(arr.iter().rev().enumerate(), &mut output);
        } else {
            print_arr(arr.iter().enumerate(), &mut output);
        }
    }

    print!("{output}");
}

fn print_arr<'a>(arr: impl Iterator<Item = (usize, &'a u8)>, output: &mut String) {
    write!(output, "[").unwrap();

    for (i, num) in arr {
        if i > 0 {
            write!(output, ",").unwrap();
        }
        write!(output, "{num}").unwrap();
    }

    writeln!(output, "]").unwrap();
}
