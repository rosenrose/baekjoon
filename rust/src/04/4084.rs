use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    while let (mut a @ 1.., mut b @ 1.., mut c @ 1.., mut d @ 1..) =
        (input(), input(), input(), input())
    {
        let mut step = 0;

        while ![b, c, d].iter().all(|&i| i == a) {
            (a, b, c, d) = (a.abs_diff(b), b.abs_diff(c), c.abs_diff(d), d.abs_diff(a));
            step += 1;
        }

        writeln!(output, "{step}").unwrap();
    }

    print!("{output}");
}
