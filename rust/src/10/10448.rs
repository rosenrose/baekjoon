use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let t = |n: i32| (n * (n + 1)) / 2;

    'outer: for k in input.skip(1) {
        for a in (1..).take_while(|&a| t(a) <= k) {
            for b in (a..).take_while(|&b| t(b) <= k - t(a)) {
                for c in (b..).take_while(|&c| t(c) <= k - t(a) - t(b)) {
                    if t(a) + t(b) + t(c) == k {
                        writeln!(output, "1").unwrap();
                        continue 'outer;
                    }
                }
            }
        }

        writeln!(output, "0").unwrap();
    }

    print!("{output}");
}
