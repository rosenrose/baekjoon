use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    for (i, n0) in (1..).map(|i| (i, input.next().unwrap())) {
        if n0 == 0 {
            break;
        }

        let n1 = 3 * n0;
        let n2 = if n1 % 2 == 0 { n1 / 2 } else { (n1 + 1) / 2 };
        let n3 = 3 * n2;

        writeln!(
            output,
            "{i}. {} {}",
            if n1 % 2 == 0 { "even" } else { "odd" },
            n3 / 9
        )
        .unwrap();
    }

    print!("{output}");
}
