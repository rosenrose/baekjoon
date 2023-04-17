use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (n, m) = (
        input.next().unwrap() as usize,
        input.next().unwrap() as usize,
    );
    let mut a: Vec<_> = input.by_ref().take(n).collect();
    let mut b: Vec<_> = input.collect();
    a.sort();
    b.sort();

    let (mut i, mut j) = (0, 0);

    while i + j < n + m {
        if i < n && j < m {
            if a[i] < b[j] {
                write!(output, "{} ", a[i]).unwrap();
                i += 1;
            } else {
                write!(output, "{} ", b[j]).unwrap();
                j += 1;
            }
        } else {
            if i == n {
                write!(output, "{} ", b[j]).unwrap();
                j += 1;
            } else {
                write!(output, "{} ", a[i]).unwrap();
                i += 1;
            }
        };
    }

    print!("{output}");
}
