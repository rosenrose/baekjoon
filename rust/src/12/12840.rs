use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (mut h, mut m, mut s) = (input(), input(), input());

    for _ in 0..input() {
        match input() {
            1 => s += input(),
            2 => s -= input(),
            3 => {
                while !matches!((h, m, s), (0..=23, 0..=59, 0..=59)) {
                    m += s / 60;
                    s %= 60;

                    h += m / 60;
                    m %= 60;

                    h %= 24;

                    if s < 0 {
                        m += s.div_euclid(60);
                        s = s.rem_euclid(60);
                    }
                    if m < 0 {
                        h += m.div_euclid(60);
                        m = m.rem_euclid(60);
                    }
                    if h < 0 {
                        h = h.rem_euclid(24);
                    }
                }

                writeln!(output, "{h} {m} {s}").unwrap();
            }
            _ => (),
        }
    }

    print!("{output}");
}
